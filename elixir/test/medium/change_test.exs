defmodule ChangeTest do
  use ExUnit.Case

  describe "returns change" do
    test "[25, 10, 5], 30" do
      assert Change.generate([5, 10, 25], 30) == {:ok, [5, 25]}
    end

    @tag :skip
    test "[1, 4, 15, 20, 50], 30" do
      assert Change.generate([1, 4, 15, 20, 50], 23) == {:ok, [4, 4, 15]}
    end

    test "another possible change without unit coins available" do
      coins = [4, 5]
      expected = [4, 4, 4, 5, 5, 5]

      assert Change.generate(coins, 27) == {:ok, expected}
    end

    test "large target values" do
      coins = [1, 2, 5, 10, 20, 50, 100]

      expected = [2, 2, 5, 20, 20, 50, 100, 100, 100, 100, 100, 100, 100, 100, 100]

      assert Change.generate(coins, 999) == {:ok, expected}
    end
  end

  describe "returns error" do
    test "error testing for change smaller than the smallest of coins" do
      coins = [5, 10]

      assert Change.generate(coins, 3) == {:error, "cannot change"}
    end

    test "error if no combination can add up to target" do
      coins = [5, 10]

      assert Change.generate(coins, 94) == {:error, "cannot change"}
    end

    test "cannot find negative change values" do
      coins = [1, 2, 5]

      assert Change.generate(coins, -5) == {:error, "cannot change"}
    end
  end
end
