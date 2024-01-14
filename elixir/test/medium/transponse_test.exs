defmodule TransponseTest do
  use ExUnit.Case

  test "empty string" do
    input = ""

    expected = ""

    assert Transpose.transpose(input) == expected
  end

  test "first line longer" do
    assert Transpose.transpose("ABC\nDE") == "AD\nBE\nC"
  end

  test "second line longer" do
    assert Transpose.transpose("AB\nDEF") == "AD\nBE\n F"
  end

  test "mixed line length" do
    input =
      "ABCDE\n" <>
        "AB\n" <>
        "ABCD\n" <>
        "A"

    expected =
      "AAAA\n" <>
        "BBB\n" <>
        "C C\n" <>
        "D D\n" <>
        "E"

    assert Transpose.transpose(input) == expected
  end

  test "triangle" do
    matrix =
      "T\n" <>
        "EE\n" <>
        "AAA\n" <>
        "SSSS\n" <>
        "EEEEE\n" <>
        "RRRRRR"

    expected =
      "TEASER\n" <>
        " EASER\n" <>
        "  ASER\n" <>
        "   SER\n" <>
        "    ER\n" <>
        "     R"

    assert Transpose.transpose(matrix) == expected
  end
end
