defmodule TripletTest do
  use ExUnit.Case

  describe "returns triplets" do
    test "12" do
      assert Triplet.generate(12) == [[3, 4, 5]]
    end

    @tag :skip
    test "840" do
      assert Triplet.generate(840) == [
               [40, 399, 401],
               [56, 390, 394],
               [105, 360, 375],
               [120, 350, 370],
               [140, 336, 364],
               [168, 315, 357],
               [210, 280, 350],
               [240, 252, 348]
             ]
    end
  end
end
