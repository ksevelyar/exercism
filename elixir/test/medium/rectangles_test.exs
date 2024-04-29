defmodule RectanglesTest do
  use ExUnit.Case

  test "no rows" do
    input = ""
    assert Rectangles.count(input) == 0
  end

  test "no columns" do
    input = """
    """

    assert Rectangles.count(input) == 0
  end

  test "no rectangles" do
    input = """
    \s
    """

    assert Rectangles.count(input) == 0
  end

  test "one rectangle" do
    input = """
    +-+
    | |
    +-+
    """

    assert Rectangles.count(input) == 1
  end

  test "two rectangles without shared parts" do
    input = """
      +-+
      | |
    +-+-+
    | | \s
    +-+ \s
    """

    assert Rectangles.count(input) == 2
  end

  test "five rectangles with shared parts" do
    input = """
      +-+
      | |
    +-+-+
    | | |
    +-+-+
    """

    assert Rectangles.count(input) == 5
  end

  test "rectangle of height 1 is counted" do
    input = """
    +--+
    +--+
    """

    assert Rectangles.count(input) == 1
  end

  test "rectangle of width 1 is counted" do
    input = """
    ++
    ||
    ++
    """

    assert Rectangles.count(input) == 1
  end

  test "1x1 square is counted" do
    input = """
    ++
    ++
    """

    assert Rectangles.count(input) == 1
  end

  test "only complete rectangles are counted" do
    input = """
      +-+
        |
    +-+-+
    | | -
    +-+-+
    """

    assert Rectangles.count(input) == 1
  end

  test "rectangles can be of different sizes" do
    input = """
    +------+----+
    |      |    |
    +---+--+    |
    |   |       |
    +---+-------+
    """

    assert Rectangles.count(input) == 3
  end

  test "corner is required for a rectangle to be complete" do
    input = """
    +------+----+
    |      |    |
    +------+    |
    |   |       |
    +---+-------+
    """

    assert Rectangles.count(input) == 2
  end

  test "large input with many rectangles" do
    input = """
    +---+--+----+
    |   +--+----+
    +---+--+    |
    |   +--+----+
    +---+--+--+-+
    +---+--+--+-+
    +------+  | |
              +-+
    """

    assert Rectangles.count(input) == 60
  end

  test "rectangles must have four sides" do
    input = """
    +-+ +-+
    | | | |
    +-+-+-+
      | | \s
    +-+-+-+
    | | | |
    +-+ +-+
    """

    assert Rectangles.count(input) == 5
  end
end
