/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */
package org.antlr.v4.test.runtime.rust;

import org.antlr.v4.Tool;
import org.antlr.v4.analysis.AnalysisPipeline;
import org.antlr.v4.automata.ATNFactory;
import org.antlr.v4.automata.ATNPrinter;
import org.antlr.v4.automata.LexerATNFactory;
import org.antlr.v4.automata.ParserATNFactory;
import org.antlr.v4.codegen.CodeGenerator;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.IntegerList;
import org.antlr.v4.runtime.misc.Interval;
import org.antlr.v4.semantics.SemanticPipeline;
import org.antlr.v4.test.runtime.BaseRuntimeTest;
import org.antlr.v4.test.runtime.ErrorQueue;
import org.antlr.v4.test.runtime.RuntimeTestSupport;
import org.antlr.v4.test.runtime.StreamVacuum;
import org.antlr.v4.tool.*;
import org.stringtemplate.v4.ST;
import org.stringtemplate.v4.STGroup;
import org.stringtemplate.v4.STGroupString;

import java.io.File;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.net.URL;
import java.util.*;

import static junit.framework.TestCase.*;
import static org.antlr.v4.test.runtime.BaseRuntimeTest.writeFile;
import static org.junit.Assert.assertArrayEquals;

public class BaseRustTest implements RuntimeTestSupport {
	public static final String newline = System.getProperty("line.separator");
	public static final String pathSep = System.getProperty("path.separator");

	/**
	 * When the {@code antlr.preserve-test-dir} runtime property is set to
	 * {@code true}, the temporary directories created by the test run will not
	 * be removed at the end of the test run, even for tests that completed
	 * successfully.
	 * <p>
	 * <p>
	 * The default behavior (used in all other cases) is removing the temporary
	 * directories for all tests which completed successfully, and preserving
	 * the directories for tests which failed.</p>
	 */
	public static final boolean PRESERVE_TEST_DIR = Boolean.parseBoolean(System.getProperty("antlr.preserve-test-dir"));

	/**
	 * The base test directory is the directory where generated files get placed
	 * during unit test execution.
	 * <p>
	 * <p>
	 * The default value for this property is the {@code java.io.tmpdir} system
	 * property, and can be overridden by setting the
	 * {@code antlr.java-test-dir} property to a custom location. Note that the
	 * {@code antlr.java-test-dir} property directly affects the
	 * {@link #CREATE_PER_TEST_DIRECTORIES} value as well.</p>
	 */
	public static final String BASE_TEST_DIR;

	/**
	 * When {@code true}, a temporary directory will be created for each test
	 * executed during the test run.
	 * <p>
	 * <p>
	 * This value is {@code true} when the {@code antlr.java-test-dir} system
	 * property is set, and otherwise {@code false}.</p>
	 */
	public static final boolean CREATE_PER_TEST_DIRECTORIES;

	public String cargo_options = "";

	static {
		String baseTestDir = System.getProperty("antlr.java-test-dir");
		boolean perTestDirectories = false;
		if (baseTestDir == null || baseTestDir.isEmpty()) {
			baseTestDir = System.getProperty("java.io.tmpdir");
			perTestDirectories = true;
		}

		if (!new File(baseTestDir).isDirectory()) {
			throw new UnsupportedOperationException("The specified base test directory does not exist: " + baseTestDir);
		}

		BASE_TEST_DIR = baseTestDir;
		CREATE_PER_TEST_DIRECTORIES = perTestDirectories;
	}

	public String tmpdir = null;
	public String outputdir = null;
	private String srcdir;

	/**
	 * If error during parser execution, store stderr here; can't return
	 * stdout and stderr.  This doesn't trap errors from running antlr.
	 */
	protected String stderrDuringParse;

	/**
	 * Errors found while running antlr
	 */
	protected StringBuilder antlrToolErrors;

	@Override
	public void testSetUp() throws Exception {
//		STGroup.verbose = true;
		if (CREATE_PER_TEST_DIRECTORIES) {
			// new output dir for each test
			String threadName = Thread.currentThread().getName();
			String testDirectory = getClass().getSimpleName() + "-" + threadName + "-" + System.nanoTime();
			tmpdir = new File(BASE_TEST_DIR, testDirectory).getAbsolutePath();

		} else {
			tmpdir = new File(BASE_TEST_DIR).getAbsolutePath();
			if (!PRESERVE_TEST_DIR && new File(tmpdir).exists()) {
				eraseFiles();
			}
		}
		// single output dir for all test which allows to not rebuild runtime every time
		outputdir = new File(BASE_TEST_DIR, "output").getAbsolutePath();
		srcdir = new File(tmpdir, "src").getAbsolutePath();

//		System.out.println(tmpdir);
		antlrToolErrors = new StringBuilder();
	}

	@Override
	public void testTearDown() throws Exception {
	}

	@Override
	public String getTmpDir() {
		return srcdir;
	}

	@Override
	public String getStdout() {
		return null;
	}

	@Override
	public String getParseErrors() {
		return stderrDuringParse;
	}

	@Override
	public String getANTLRToolErrors() {
		if (antlrToolErrors.length() == 0) {
			return null;
		}
		return antlrToolErrors.toString();
	}

	protected Tool newTool(String[] args) {
		Tool tool = new Tool(args);
		return tool;
	}

	protected ATN createATN(Grammar g, boolean useSerializer) {
		if (g.atn == null) {
			semanticProcess(g);
			assertEquals(0, g.tool.getNumErrors());

			ParserATNFactory f;
			if (g.isLexer()) {
				f = new LexerATNFactory((LexerGrammar) g);
			} else {
				f = new ParserATNFactory(g);
			}

			g.atn = f.createATN();
			assertEquals(0, g.tool.getNumErrors());
		}

		ATN atn = g.atn;
		if (useSerializer) {
			char[] serialized = ATNSerializer.getSerializedAsChars(atn);
			return new ATNDeserializer().deserialize(serialized);
		}

		return atn;
	}

	protected void semanticProcess(Grammar g) {
		if (g.ast != null && !g.ast.hasErrors) {
//			System.out.println(g.ast.toStringTree());
			Tool antlr = new Tool();
			SemanticPipeline sem = new SemanticPipeline(g);
			sem.process();
			if (g.getImportedGrammars() != null) { // process imported grammars (if any)
				for (Grammar imp : g.getImportedGrammars()) {
					antlr.processNonCombinedGrammar(imp, false);
				}
			}
		}
	}

	public DFA createDFA(Grammar g, DecisionState s) {
//		PredictionDFAFactory conv = new PredictionDFAFactory(g, s);
//		DFA dfa = conv.createDFA();
//		conv.issueAmbiguityWarnings();
//		System.out.print("DFA="+dfa);
//		return dfa;
		return null;
	}

//	public void minimizeDFA(DFA dfa) {
//		DFAMinimizer dmin = new DFAMinimizer(dfa);
//		dfa.minimized = dmin.minimize();
//	}

	IntegerList getTypesFromString(Grammar g, String expecting) {
		IntegerList expectingTokenTypes = new IntegerList();
		if (expecting != null && !expecting.trim().isEmpty()) {
			for (String tname : expecting.replace(" ", "").split(",")) {
				int ttype = g.getTokenType(tname);
				expectingTokenTypes.add(ttype);
			}
		}
		return expectingTokenTypes;
	}

	public IntegerList getTokenTypesViaATN(String input, LexerATNSimulator lexerATN) {
		ANTLRInputStream in = new ANTLRInputStream(input);
		IntegerList tokenTypes = new IntegerList();
		int ttype;
		do {
			ttype = lexerATN.match(in, Lexer.DEFAULT_MODE);
			tokenTypes.add(ttype);
		} while (ttype != Token.EOF);
		return tokenTypes;
	}

	public List<String> getTokenTypes(LexerGrammar lg,
									  ATN atn,
									  CharStream input) {
		LexerATNSimulator interp = new LexerATNSimulator(atn, new DFA[]{new DFA(atn.modeToStartState.get(Lexer.DEFAULT_MODE))}, null);
		List<String> tokenTypes = new ArrayList<String>();
		int ttype;
		boolean hitEOF = false;
		do {
			if (hitEOF) {
				tokenTypes.add("EOF");
				break;
			}
			int t = input.LA(1);
			ttype = interp.match(input, Lexer.DEFAULT_MODE);
			if (ttype == Token.EOF) {
				tokenTypes.add("EOF");
			} else {
				tokenTypes.add(lg.typeToTokenList.get(ttype));
			}

			if (t == IntStream.EOF) {
				hitEOF = true;
			}
		} while (ttype != Token.EOF);
		return tokenTypes;
	}

	List<ANTLRMessage> checkRuleDFA(String gtext, String ruleName, String expecting)
			throws Exception {
		ErrorQueue equeue = new ErrorQueue();
		Grammar g = new Grammar(gtext, equeue);
		ATN atn = createATN(g, false);
		ATNState s = atn.ruleToStartState[g.getRule(ruleName).index];
		if (s == null) {
			System.err.println("no such rule: " + ruleName);
			return null;
		}
		ATNState t = s.transition(0).target;
		if (!(t instanceof DecisionState)) {
			System.out.println(ruleName + " has no decision");
			return null;
		}
		DecisionState blk = (DecisionState) t;
		checkRuleDFA(g, blk, expecting);
		return equeue.all;
	}

	List<ANTLRMessage> checkRuleDFA(String gtext, int decision, String expecting)
			throws Exception {
		ErrorQueue equeue = new ErrorQueue();
		Grammar g = new Grammar(gtext, equeue);
		ATN atn = createATN(g, false);
		DecisionState blk = atn.decisionToState.get(decision);
		checkRuleDFA(g, blk, expecting);
		return equeue.all;
	}

	void checkRuleDFA(Grammar g, DecisionState blk, String expecting)
			throws Exception {
		DFA dfa = createDFA(g, blk);
		String result = null;
		if (dfa != null) result = dfa.toString();
		assertEquals(expecting, result);
	}

	List<ANTLRMessage> checkLexerDFA(String gtext, String expecting)
			throws Exception {
		return checkLexerDFA(gtext, LexerGrammar.DEFAULT_MODE_NAME, expecting);
	}

	List<ANTLRMessage> checkLexerDFA(String gtext, String modeName, String expecting)
			throws Exception {
		ErrorQueue equeue = new ErrorQueue();
		LexerGrammar g = new LexerGrammar(gtext, equeue);
		g.atn = createATN(g, false);
//		LexerATNToDFAConverter conv = new LexerATNToDFAConverter(g);
//		DFA dfa = conv.createDFA(modeName);
//		g.setLookaheadDFA(0, dfa); // only one decision to worry about
//
//		String result = null;
//		if ( dfa!=null ) result = dfa.toString();
//		assertEquals(expecting, result);
//
//		return equeue.all;
		return null;
	}

	protected String execLexer(String grammarFileName, String grammarStr,
							   String lexerName, String input) {
		return execLexer(grammarFileName, grammarStr, lexerName, input, false);
	}

	@Override
	public String execLexer(String grammarFileName, String grammarStr,
							String lexerName, String input, boolean showDFA) {
		boolean success = rawGenerateAndBuildRecognizer(grammarFileName,
				grammarStr, null, lexerName, "-no-listener");
		assertTrue(success);
		writeFile(tmpdir, "input", input);
		writeLexerTestFile(lexerName, showDFA);
		return executeRecognizer();
	}

	@Override
	public String execParser(String grammarFileName,
							 String grammarStr,
							 String parserName,
							 String lexerName,
							 String listenerName,
							 String visitorName,
							 String startRuleName,
							 String input,
							 boolean showDiagnosticErrors) {
		return execParser(grammarFileName, grammarStr, parserName, lexerName,
				listenerName, visitorName, startRuleName, input, showDiagnosticErrors, false);
	}

	/**
	 * ANTLR isn't thread-safe to process grammars so we use a global lock for testing
	 */
	public static final Object antlrLock = new Object();

	public String execParser(String grammarFileName,
							 String grammarStr,
							 String parserName,
							 String lexerName,
							 String listenerName,
							 String visitorName,
							 String startRuleName,
							 String input,
							 boolean showDiagnosticErrors,
							 boolean profile) {
		boolean success = rawGenerateAndBuildRecognizer(grammarFileName,
				grammarStr,
				parserName,
				lexerName,
				"-visitor");
		assertTrue(success);
		writeFile(tmpdir, "input", input);
		rawBuildRecognizerTestFile(parserName, lexerName, listenerName,
				visitorName, startRuleName, showDiagnosticErrors);
		return executeRecognizer();
	}

	protected void rawBuildRecognizerTestFile(String parserName,
											  String lexerName, String listenerName, String visitorName,
											  String parserStartRuleName, boolean debug) {
		this.stderrDuringParse = null;
		if (parserName == null) {
			writeLexerTestFile(lexerName, false);
		} else {
			writeParserTestFile(parserName, lexerName, listenerName,
					visitorName, parserStartRuleName, debug);
		}
	}

	/**
	 * Return true if all is well
	 */
	protected boolean rawGenerateAndBuildRecognizer(String grammarFileName,
													String grammarStr,
													String parserName,
													String lexerName,
													String... extraOptions) {
		return rawGenerateAndBuildRecognizer(grammarFileName, grammarStr, parserName, lexerName, false, extraOptions);
	}

	/**
	 * Return true if all is well
	 */
	protected boolean rawGenerateAndBuildRecognizer(String grammarFileName,
													String grammarStr,
													String parserName,
													String lexerName,
													boolean defaultListener,
													String... extraOptions) {
		ErrorQueue equeue =
				BaseRuntimeTest.antlrOnString(getTmpDir(), "Rust", grammarFileName, grammarStr, defaultListener, extraOptions);
		if (!equeue.errors.isEmpty()) {
			System.out.println(equeue.errors);
			return false;
		}
		return true;
	}

	private String executeRecognizer() {
//		System.out.println("dir: " + tmpdir);
		writeFile(tmpdir, "Cargo.toml",
				"[package]\n" +
						"name = \"antlr-test\"\n" +
						"version=\"0.1.0\"\n" +
						"edition=\"2018\"\n" +
						"\n" +
						"[dependencies]\n" +
						"antlr-rust = { path = \"" + locateRuntimeSrc() + "\" }\n" +
						"[profile.release]\n" +
						"opt-level=1\n");

		cargo("build");
		this.stderrDuringParse = null;

		return cargo("run");
	}

	private String locateRuntimeSrc() {
		ClassLoader loader = Thread.currentThread().getContextClassLoader();
		URL rustRuntime = loader.getResource("Rust");
		if (rustRuntime == null) {
			throw new RuntimeException("Rust runtime file not found at:" + rustRuntime.getPath());
		}
		File runtimeDir = new File(rustRuntime.getPath());
		if (!runtimeDir.exists()) {
			throw new RuntimeException("Cannot find Rust ANTLR runtime");
		}

		return runtimeDir.getAbsolutePath();
	}

	private String cargo(String command) {
		try {
			ProcessBuilder builder = new ProcessBuilder("cargo", command, "--quiet"/*, "--offline"*/, cargo_options);
			builder.environment().put("CARGO_TARGET_DIR", outputdir);
			builder.environment().put("RUST_BACKTRACE", "1");
			builder.environment().put("RUSTFLAGS", "-Awarnings");
			builder.directory(new File(tmpdir));
			long time = System.currentTimeMillis();
			Process process = builder.start();
			StreamVacuum stdoutVacuum = new StreamVacuum(process.getInputStream());
			StreamVacuum stderrVacuum = new StreamVacuum(process.getErrorStream());
			stdoutVacuum.start();
			stderrVacuum.start();
			int rc = process.waitFor();
//			System.out.println("cargo " + command + ", exec time: " + (System.currentTimeMillis() - time));
			stdoutVacuum.join();
			stderrVacuum.join();
			String output = stdoutVacuum.toString();
			if (output.length() == 0) {
				output = null;
			}
			if (stderrVacuum.toString().length() > 0) {
				stderrDuringParse = stderrVacuum.toString();
			}
			return output;
		} catch (Exception e) {
			System.err.println("can't exec recognizer");
			e.printStackTrace(System.err);
			e.printStackTrace(System.out);
			StringWriter sw = new StringWriter();
			PrintWriter pw = new PrintWriter(sw);
			e.printStackTrace(pw);
			return sw.toString();
		}
//		return null;
	}

	List<ANTLRMessage> getMessagesOfType(List<ANTLRMessage> msgs, Class<? extends ANTLRMessage> c) {
		List<ANTLRMessage> filtered = new ArrayList<ANTLRMessage>();
		for (ANTLRMessage m : msgs) {
			if (m.getClass() == c) filtered.add(m);
		}
		return filtered;
	}

	public void checkRuleATN(Grammar g, String ruleName, String expecting) {
//		DOTGenerator dot = new DOTGenerator(g);
//		System.out.println(dot.getDOT(g.atn.ruleToStartState[g.getRule(ruleName).index]));

		Rule r = g.getRule(ruleName);
		ATNState startState = g.getATN().ruleToStartState[r.index];
		ATNPrinter serializer = new ATNPrinter(g, startState);
		String result = serializer.asString();

		//System.out.print(result);
		assertEquals(expecting, result);
	}

	public void testActions(String templates, String actionName, String action, String expected) throws org.antlr.runtime.RecognitionException {
		int lp = templates.indexOf('(');
		String name = templates.substring(0, lp);
		STGroup group = new STGroupString(templates);
		ST st = group.getInstanceOf(name);
		st.add(actionName, action);
		String grammar = st.render();
		ErrorQueue equeue = new ErrorQueue();
		Grammar g = new Grammar(grammar, equeue);
		if (g.ast != null && !g.ast.hasErrors) {
			SemanticPipeline sem = new SemanticPipeline(g);
			sem.process();

			ATNFactory factory = new ParserATNFactory(g);
			if (g.isLexer()) factory = new LexerATNFactory((LexerGrammar) g);
			g.atn = factory.createATN();

			AnalysisPipeline anal = new AnalysisPipeline(g);
			anal.process();

			CodeGenerator gen = new CodeGenerator(g);
			ST outputFileST = gen.generateParser(false);
			String output = outputFileST.render();
			//System.out.println(output);
			String b = "#" + actionName + "#";
			int start = output.indexOf(b);
			String e = "#end-" + actionName + "#";
			int end = output.indexOf(e);
			String snippet = output.substring(start + b.length(), end);
			assertEquals(expected, snippet);
		}
		if (equeue.size() > 0) {
//			System.err.println(equeue.toString());
		}
	}

	protected void checkGrammarSemanticsError(ErrorQueue equeue,
											  GrammarSemanticsMessage expectedMessage)
			throws Exception {
		ANTLRMessage foundMsg = null;
		for (int i = 0; i < equeue.errors.size(); i++) {
			ANTLRMessage m = equeue.errors.get(i);
			if (m.getErrorType() == expectedMessage.getErrorType()) {
				foundMsg = m;
			}
		}
		assertNotNull("no error; " + expectedMessage.getErrorType() + " expected", foundMsg);
		assertTrue("error is not a GrammarSemanticsMessage",
				foundMsg instanceof GrammarSemanticsMessage);
		assertEquals(Arrays.toString(expectedMessage.getArgs()), Arrays.toString(foundMsg.getArgs()));
		if (equeue.size() != 1) {
			System.err.println(equeue);
		}
	}

	protected void checkGrammarSemanticsWarning(ErrorQueue equeue,
												GrammarSemanticsMessage expectedMessage)
			throws Exception {
		ANTLRMessage foundMsg = null;
		for (int i = 0; i < equeue.warnings.size(); i++) {
			ANTLRMessage m = equeue.warnings.get(i);
			if (m.getErrorType() == expectedMessage.getErrorType()) {
				foundMsg = m;
			}
		}
		assertNotNull("no error; " + expectedMessage.getErrorType() + " expected", foundMsg);
		assertTrue("error is not a GrammarSemanticsMessage",
				foundMsg instanceof GrammarSemanticsMessage);
		assertEquals(Arrays.toString(expectedMessage.getArgs()), Arrays.toString(foundMsg.getArgs()));
		if (equeue.size() != 1) {
			System.err.println(equeue);
		}
	}

	protected void checkError(ErrorQueue equeue,
							  ANTLRMessage expectedMessage)
			throws Exception {
		//System.out.println("errors="+equeue);
		ANTLRMessage foundMsg = null;
		for (int i = 0; i < equeue.errors.size(); i++) {
			ANTLRMessage m = equeue.errors.get(i);
			if (m.getErrorType() == expectedMessage.getErrorType()) {
				foundMsg = m;
			}
		}
		assertTrue("no error; " + expectedMessage.getErrorType() + " expected", !equeue.errors.isEmpty());
		assertTrue("too many errors; " + equeue.errors, equeue.errors.size() <= 1);
		assertNotNull("couldn't find expected error: " + expectedMessage.getErrorType(), foundMsg);
		/*
		assertTrue("error is not a GrammarSemanticsMessage",
				   foundMsg instanceof GrammarSemanticsMessage);
		 */
		assertArrayEquals(expectedMessage.getArgs(), foundMsg.getArgs());
	}

	public static class FilteringTokenStream extends CommonTokenStream {
		public FilteringTokenStream(TokenSource src) {
			super(src);
		}

		Set<Integer> hide = new HashSet<Integer>();

		@Override
		protected boolean sync(int i) {
			if (!super.sync(i)) {
				return false;
			}

			Token t = get(i);
			if (hide.contains(t.getType())) {
				((WritableToken) t).setChannel(Token.HIDDEN_CHANNEL);
			}

			return true;
		}

		public void setTokenTypeChannel(int ttype, int channel) {
			hide.add(ttype);
		}
	}

	protected void writeParserTestFile(String parserName, String lexerName,
									   String listenerName, String visitorName,
									   String parserStartRuleName, boolean debug) {
		ST outputFileST = new ST(
//				"#![feature(try_blocks)]\n" +
//				"#![feature(inner_deref)]\n" +
//				"#![feature(specialization)]\n" +
				"mod <importLexer>;\n" +
				"use <importLexer>::*;\n" +
				"mod <importParser>;\n" +
				"use <importParser>::*;\n" +
				"mod <importListener>;\n" +
				"mod <importVisitor>;\n" +
				"use antlr_rust::InputStream;\n" +
				"use antlr_rust::token::OwningToken;\n" +
				"use antlr_rust::token_stream::{UnbufferedTokenStream, TokenStream};\n" +
				"use antlr_rust::common_token_stream::CommonTokenStream;\n" +
				"use antlr_rust::parser::Parser;\n" +
				"use antlr_rust::error_listener::DiagnosticErrorListener;\n" +
				"\n" +
				"fn main() -> std::io::Result\\<()>{\n" +
				"	let input = std::fs::read_to_string(std::env::current_dir()?.join(\"input\"))?;\n" +
				"	let input = input.chars().map(|x|x as u32).collect::\\<Vec\\<_> >();\n" +
				"	let mut lexer = <lexerName>::new(InputStream::new(&*input));\n" +
				"	let mut token_source = CommonTokenStream::new(lexer);\n" +
				"<createParser>" +
				"	let result = parser.<parserStartRuleName>().unwrap();\n" +
				"	\n" +
				"	Ok(())" +
				"}\n"
		);
		ST createParserST = new ST("    let mut parser = <parserName>::with_dyn_strategy(token_source);\n");
		if (debug) {
			createParserST =
					new ST(
							"    let mut parser = <parserName>::with_dyn_strategy(token_source);\n" +
									"    parser.add_error_listener(Box::new(DiagnosticErrorListener::new(true)));\n");
		}
		outputFileST.add("createParser", createParserST);
		outputFileST.add("parserName", parserName);
		outputFileST.add("importParser", parserName.toLowerCase());
		outputFileST.add("lexerName", lexerName);
		outputFileST.add("importLexer", lexerName.toLowerCase());
		outputFileST.add("importListener", listenerName.toLowerCase());
		outputFileST.add("importVisitor", visitorName.toLowerCase());
		outputFileST.add("parserStartRuleName", parserStartRuleName);
		writeFile(srcdir, "main.rs", outputFileST.render());
	}


	protected void writeLexerTestFile(String lexerName, boolean showDFA) {
		ST outputFileST = new ST("use antlr_rust::*;\n" +
				"mod <importName>;\n" +
				"use <importName>::*;\n" +
				"use antlr_rust::InputStream;\n" +
				"use antlr_rust::lexer::Lexer;\n" +
				"use antlr_rust::token::OwningToken;\n" +
				"use antlr_rust::token_stream::{UnbufferedTokenStream, TokenStream};\n" +
				"\n" +
				"fn main() -> std::io::Result\\<()>{\n" +
				"	let input = std::fs::read_to_string(std::env::current_dir()?.join(\"input\"))?;\n" +
				"	let input = input.chars().map(|x|x as u32).collect::\\<Vec\\<_> >();\n" +
				"	let mut _lexer = <lexerName>::new(InputStream::new(&*input));\n" +
				"	{let mut token_source = UnbufferedTokenStream::new_unbuffered(&mut _lexer);\n" +
				"		let tokens = token_source.token_iter().collect::\\<Vec\\<OwningToken>>();\n" +
				"		for token in tokens.iter(){\n" +
				"			println!(\"{}\",token);\n" +
				"		}\n" +
				"	}\n" +
				(showDFA ?
						"print!(\"{}\",_lexer.get_interpreter().unwrap()" +
								".get_dfa_for_mode(antlr_rust::lexer::LEXER_DEFAULT_MODE)" +
								".read().to_lexer_string());\n"
						: "") +
				"	Ok(())" +
				"}\n"
		);

		outputFileST.add("lexerName", lexerName);
		if (lexerName.endsWith("Lexer")) {
			outputFileST.add("importName", lexerName.toLowerCase());
		} else {
			outputFileST.add("importName", lexerName.toLowerCase() + "lexer");
		}
		writeFile(srcdir, "main.rs", outputFileST.render());
	}

	protected void eraseFiles(final String filesEndingWith) {
		File tmpdirF = new File(tmpdir);
		String[] files = tmpdirF.list();
		for (int i = 0; files != null && i < files.length; i++) {
			if (files[i].endsWith(filesEndingWith)) {
				new File(tmpdir + "/" + files[i]).delete();
			}
		}
	}

	protected void eraseFiles() {
		if (tmpdir == null) {
			return;
		}

		File tmpdirF = new File(tmpdir);
		String[] files = tmpdirF.list();
		for (int i = 0; files != null && i < files.length; i++) {
			new File(tmpdir + "/" + files[i]).delete();
		}
	}

	public void eraseTempDir() {
		File tmpdirF = new File(tmpdir);
		if (tmpdirF.exists()) {
			eraseFiles();
			tmpdirF.delete();
		}
	}

	public String getFirstLineOfException() {
		if (this.stderrDuringParse == null) {
			return null;
		}
		String[] lines = this.stderrDuringParse.split("\n");
		String prefix = "Exception in thread \"main\" ";
		return lines[0].substring(prefix.length(), lines[0].length());
	}

	/**
	 * When looking at a result set that consists of a Map/HashTable
	 * we cannot rely on the output order, as the hashing algorithm or other aspects
	 * of the implementation may be different on differnt JDKs or platforms. Hence
	 * we take the Map, convert the keys to a List, sort them and Stringify the Map, which is a
	 * bit of a hack, but guarantees that we get the same order on all systems. We assume that
	 * the keys are strings.
	 *
	 * @param m The Map that contains keys we wish to return in sorted order
	 * @return A string that represents all the keys in sorted order.
	 */
	public <K, V> String sortMapToString(Map<K, V> m) {
		// Pass in crap, and get nothing back
		//
		if (m == null) {
			return null;
		}

		System.out.println("Map toString looks like: " + m.toString());

		// Sort the keys in the Map
		//
		TreeMap<K, V> nset = new TreeMap<K, V>(m);

		System.out.println("Tree map looks like: " + nset.toString());
		return nset.toString();
	}

	public List<String> realElements(List<String> elements) {
		return elements.subList(Token.MIN_USER_TOKEN_TYPE, elements.size());
	}

	public void assertNotNullOrEmpty(String message, String text) {
		assertNotNull(message, text);
		assertFalse(message, text.isEmpty());
	}

	public void assertNotNullOrEmpty(String text) {
		assertNotNull(text);
		assertFalse(text.isEmpty());
	}

	public static class IntTokenStream implements TokenStream {
		public IntegerList types;
		int p = 0;

		public IntTokenStream(IntegerList types) {
			this.types = types;
		}

		@Override
		public void consume() {
			p++;
		}

		@Override
		public int LA(int i) {
			return LT(i).getType();
		}

		@Override
		public int mark() {
			return index();
		}

		@Override
		public int index() {
			return p;
		}

		@Override
		public void release(int marker) {
			seek(marker);
		}

		@Override
		public void seek(int index) {
			p = index;
		}

		@Override
		public int size() {
			return types.size();
		}

		@Override
		public String getSourceName() {
			return UNKNOWN_SOURCE_NAME;
		}

		@Override
		public Token LT(int i) {
			CommonToken t;
			int rawIndex = p + i - 1;
			if (rawIndex >= types.size()) t = new CommonToken(Token.EOF);
			else t = new CommonToken(types.get(rawIndex));
			t.setTokenIndex(rawIndex);
			return t;
		}

		@Override
		public Token get(int i) {
			return new CommonToken(types.get(i));
		}

		@Override
		public TokenSource getTokenSource() {
			return null;
		}


		@Override
		public String getText() {
			throw new UnsupportedOperationException("can't give strings");
		}


		@Override
		public String getText(Interval interval) {
			throw new UnsupportedOperationException("can't give strings");
		}


		@Override
		public String getText(RuleContext ctx) {
			throw new UnsupportedOperationException("can't give strings");
		}


		@Override
		public String getText(Token start, Token stop) {
			throw new UnsupportedOperationException("can't give strings");
		}
	}

	/**
	 * Sort a list
	 */
	public <T extends Comparable<? super T>> List<T> sort(List<T> data) {
		List<T> dup = new ArrayList<T>();
		dup.addAll(data);
		Collections.sort(dup);
		return dup;
	}

	/**
	 * Return map sorted by key
	 */
	public <K extends Comparable<? super K>, V> LinkedHashMap<K, V> sort(Map<K, V> data) {
		LinkedHashMap<K, V> dup = new LinkedHashMap<K, V>();
		List<K> keys = new ArrayList<K>();
		keys.addAll(data.keySet());
		Collections.sort(keys);
		for (K k : keys) {
			dup.put(k, data.get(k));
		}
		return dup;
	}
}
