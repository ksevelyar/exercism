defmodule TripletTest do
  use ExUnit.Case

  describe "returns triplets" do
    test "12" do
      assert Triplet.generate(12) == [[3, 4, 5]]
    end

    test "108" do
      assert Triplet.generate(108) == [[27, 36, 45]]
    end

    test "90" do
      assert Triplet.generate(90) == [[9, 40, 41], [15, 36, 39]]
    end

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
