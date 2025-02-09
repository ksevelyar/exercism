defmodule SgfParsingTest do
  alias SgfParsing.Sgf

  use ExUnit.Case

  test "empty input" do
    encoded = ""

    output = SgfParsing.parse(encoded)

    expected = {:error, "tree missing"}

    assert output == expected
  end

  test "tree with no nodes" do
    encoded = "()"

    output = SgfParsing.parse(encoded)

    expected = {:error, "tree with no nodes"}

    assert output == expected
  end

  test "node without tree" do
    encoded = ";"

    output = SgfParsing.parse(encoded)

    expected = {:error, "tree missing"}

    assert output == expected
  end

  test "node without properties" do
    encoded = "(;)"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{}}

    assert output == expected
  end

  test "single node tree" do
    encoded = "(;A[B])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["B"]}}}

    assert output == expected
  end

  test "multiple properties" do
    encoded = "(;A[b]C[d])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["b"], "C" => ["d"]}}}

    assert output == expected
  end

  test "properties without delimiter" do
    encoded = "(;A)"

    output = SgfParsing.parse(encoded)

    expected = {:error, "properties without delimiter"}

    assert output == expected
  end

  test "all lowercase property" do
    encoded = "(;a[b])"

    output = SgfParsing.parse(encoded)

    expected = {:error, "property must be in uppercase"}

    assert output == expected
  end

  test "upper and lowercase property" do
    encoded = "(;Aa[b])"

    output = SgfParsing.parse(encoded)

    expected = {:error, "property must be in uppercase"}

    assert output == expected
  end

  test "two nodes" do
    encoded = "(;A[B];B[C])"

    output = SgfParsing.parse(encoded)

    expected =
      {:ok, %Sgf{children: [%Sgf{properties: %{"B" => ["C"]}}], properties: %{"A" => ["B"]}}}

    assert output == expected
  end

  test "two child trees" do
    encoded = "(;A[B](;B[C])(;C[D]))"

    output = SgfParsing.parse(encoded)

    expected =
      {:ok,
       %Sgf{
         children: [
           %Sgf{properties: %{"B" => ["C"]}},
           %Sgf{properties: %{"C" => ["D"]}}
         ],
         properties: %{"A" => ["B"]}
       }}

    assert output == expected
  end

  test "multiple property values" do
    encoded = "(;A[b][c][d])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["b", "c", "d"]}}}

    assert output == expected
  end

  test "within property values, whitespace characters such as tab are converted to spaces" do
    encoded = "(;A[hello\t\tworld])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["hello  world"]}}}

    assert output == expected
  end

  test "within property values, newlines remain as newlines" do
    encoded = "(;A[hello\n\nworld])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["hello\n\nworld"]}}}

    assert output == expected
  end

  test "escaped closing bracket within property value becomes just a closing bracket" do
    encoded = "(;A[\\]])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["]"]}}}

    assert output == expected
  end

  test "escaped backslash in property value becomes just a backslash" do
    encoded = "(;A[\\\\])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["\\"]}}}

    assert output == expected
  end

  test "opening bracket within property value doesn't need to be escaped" do
    encoded = "(;A[x[y\\]z][foo]B[bar];C[baz])"

    output = SgfParsing.parse(encoded)

    expected =
      {:ok,
       %Sgf{
         properties: %{"A" => ["x[y]z", "foo"], "B" => ["bar"]},
         children: [
           %Sgf{properties: %{"C" => ["baz"]}}
         ]
       }}

    assert output == expected
  end

  test "semicolon in property value doesn't need to be escaped" do
    encoded = "(;A[a;b][foo]B[bar];C[baz])"

    output = SgfParsing.parse(encoded)

    expected =
      {:ok,
       %Sgf{
         properties: %{"A" => ["a;b", "foo"], "B" => ["bar"]},
         children: [
           %Sgf{properties: %{"C" => ["baz"]}}
         ]
       }}

    assert output == expected
  end

  test "parentheses in property value don't need to be escaped" do
    encoded = "(;A[x(y)z][foo]B[bar];C[baz])"

    output = SgfParsing.parse(encoded)

    expected =
      {:ok,
       %Sgf{
         properties: %{"A" => ["x(y)z", "foo"], "B" => ["bar"]},
         children: [
           %Sgf{properties: %{"C" => ["baz"]}}
         ]
       }}

    assert output == expected
  end

  test "escaped tab in property value is converted to space" do
    encoded = "(;A[hello\\\tworld])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["hello world"]}}}

    assert output == expected
  end

  test "escaped newline in property value is converted to nothing at all" do
    encoded = "(;A[hello\\\nworld])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["helloworld"]}}}

    assert output == expected
  end

  test "escaped t and n in property value are just letters, not whitespace" do
    encoded = "(;A[\\t = t and \\n = n])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["t = t and n = n"]}}}

    assert output == expected
  end

  test "mixing various kinds of whitespace and escaped characters in property value" do
    encoded = "(;A[\\]b\nc\\\nd\t\te\\\\ \\\n\\]])"

    output = SgfParsing.parse(encoded)

    expected = {:ok, %Sgf{properties: %{"A" => ["]b\ncd  e\\ ]"]}}}

    assert output == expected
  end
end
