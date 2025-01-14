defmodule ZebraPuzzleTest do
  use ExUnit.Case

  test "resident who drinks water" do
    assert ZebraPuzzle.drinks_water() == :norwegian
  end

  test "resident who owns zebra" do
    assert ZebraPuzzle.owns_zebra() == :japanese
  end
end
