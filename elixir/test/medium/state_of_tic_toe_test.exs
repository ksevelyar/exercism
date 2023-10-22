defmodule StateOfTicTacToeTest do
  use ExUnit.Case

  describe "Won games" do
    @tag :skip
    test "Finished game where X won via left column victory" do
      board = """
      XOO
      X..
      X..
      """

      assert StateOfTicTacToe.game_state(board) == {:ok, :win}
    end
  end
end
