defmodule BowlingTest do
  use ExUnit.Case

  defp roll_reduce(game, rolls) do
    Enum.reduce(rolls, game, fn roll, game ->
      {:ok, game} = Bowling.roll(game, roll)

      game
    end)
  end

  test "should be able to score a game with all zeros" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 0}
  end

  test "should be able to score a game with no strikes or spares" do
    game = Bowling.start()

    rolls = [3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6, 3, 6]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 90}
  end

  test "a spare followed by zeros is worth ten points" do
    game = Bowling.start()

    rolls = [6, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 10}
  end

  test "points scored in the roll after a spare are counted twice" do
    game = Bowling.start()

    rolls = [6, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 16}
  end

  test "consecutive spares each get a one roll bonus" do
    game = Bowling.start()

    rolls = [5, 5, 3, 7, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 31}
  end

  test "a spare in the last frame gets a one roll bonus that is counted once" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 3, 7]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 17}
  end

  test "a strike earns ten points in a frame with a single roll" do
    game = Bowling.start()

    rolls = [10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 10}
  end

  test "points scored in the two rolls after a strike are counted twice as a bonus" do
    game = Bowling.start()

    rolls = [10, 5, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 26}
  end

  test "consecutive strikes each get the two roll bonus" do
    game = Bowling.start()

    rolls = [10, 10, 10, 5, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 81}
  end

  test "a strike in the last frame gets a two roll bonus that is counted once" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 7, 1]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 18}
  end

  test "rolling a spare with the two roll bonus does not get a bonus roll" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 7, 3]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 20}
  end

  test "strikes with the two roll bonus do not get bonus rolls" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 30}
  end

  test "last two strikes followed by only last bonus with non strike points" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 1]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 31}
  end

  test "a strike with the one roll bonus after a spare in the last frame does not get a bonus" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 3, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 20}
  end

  test "all strikes is a perfect game" do
    game = Bowling.start()

    rolls = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 300}
  end

  test "rolls cannot score negative points" do
    game = Bowling.start()

    assert Bowling.roll(game, -1) == {:error, "Negative roll is invalid"}
  end

  test "a roll cannot score more than 10 points" do
    game = Bowling.start()

    assert Bowling.roll(game, 11) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "two rolls in a frame cannot score more than 10 points" do
    game = Bowling.start()

    {:ok, game} = Bowling.roll(game, 5)

    assert Bowling.roll(game, 6) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "bonus roll after a strike in the last frame cannot score more than 10 points" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 11) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "two bonus rolls after a strike in the last frame cannot score more than 10 points" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 5]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 6) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "two bonus rolls after a strike in the last frame can score more than 10 points if one is a strike" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 6]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:ok, 26}
  end

  test "the second bonus rolls after a strike in the last frame cannot be a strike if the first one is not a strike" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 6]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 10) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "second bonus roll after a strike in the last frame cannot score more than 10 points" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 11) == {:error, "Pin count exceeds pins on the lane"}
  end

  test "an unstarted game cannot be scored" do
    game = Bowling.start()

    assert Bowling.score(game) == {:error, "Score cannot be taken until the end of the game"}
  end

  test "an incomplete game cannot be scored" do
    game = Bowling.start()

    rolls = [0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:error, "Score cannot be taken until the end of the game"}
  end

  test "cannot roll if game already has ten frames" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 0) == {:error, "Cannot roll after game is over"}
  end

  test "bonus rolls for a strike in the last frame must be rolled before score can be calculated" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:error, "Score cannot be taken until the end of the game"}
  end

  test "both bonus rolls for a strike in the last frame must be rolled before score can be calculated" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:error, "Score cannot be taken until the end of the game"}
  end

  test "bonus roll for a spare in the last frame must be rolled before score can be calculated" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 3]

    game = roll_reduce(game, rolls)

    assert Bowling.score(game) == {:error, "Score cannot be taken until the end of the game"}
  end

  test "cannot roll after bonus roll for spare" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 3, 2]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 2) == {:error, "Cannot roll after game is over"}
  end

  test "cannot roll after bonus roll for strike" do
    game = Bowling.start()

    rolls = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 3, 2]

    game = roll_reduce(game, rolls)

    assert Bowling.roll(game, 2) == {:error, "Cannot roll after game is over"}
  end
end
