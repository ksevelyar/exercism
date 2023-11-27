defmodule Queens do
  @type t :: %Queens{black: {integer, integer}, white: {integer, integer}}
  defstruct [:white, :black]

  @doc """
  Creates a new set of Queens
  """
  @spec new(Keyword.t()) :: Queens.t()
  def new(opts \\ []) do
    %__MODULE__{white: Keyword.get(opts, :white), black: Keyword.get(opts, :black)}
  end

  @doc """
  Gives a string representation of the board with
  white and black queen locations shown
  """
  @spec to_string(Queens.t()) :: String.t()
  def to_string(queens) do
  end

  @doc """
  Checks if the queens can attack each other
  """
  @spec can_attack?(Queens.t()) :: boolean
  def can_attack?(%{white: {wc, wr}, black: {bc, br}}) do
    wc == bc || wr == br || same_diagonal?({wc, wr}, {bc, br})
  end

  def can_attack?(_), do: false

  defp diagonal1(x, y) when x < y, do: {0, y - x}
  defp diagonal1(x, y), do: {x - y, 0}

  defp diagonal2(x, y) when x + y > 7, do: {7, 7 - x + y}
  defp diagonal2(x, y), do: {x + y, 0}

  defp same_diagonal?({wc, wr}, {bc, br}) do
    diagonal1(wc, wr) == diagonal1(bc, br) || diagonal2(wc, wr) == diagonal2(bc, br)
  end
end
