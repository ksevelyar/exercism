defmodule Grains do
  @chessboard_range 1..64

  @doc """
  Calculate two to the power of the input minus one.
  """
  @spec square(pos_integer()) :: {:ok, pos_integer()} | {:error, String.t()}
  def square(number) when number in @chessboard_range do
    {:ok, 2 ** (number - 1)}
  end
  def square(_number) do
    {:error, "The requested square must be between 1 and 64 (inclusive)"}
  end

  @doc """
  Adds square of each number from 1 to 64.
  """
  @spec total :: {:ok, pos_integer()}
  def total do
    total =
      @chessboard_range
      |> Enum.map(fn number -> number |> square() |> elem(1) end)
      |> Enum.sum()

    {:ok, total}
  end
end
