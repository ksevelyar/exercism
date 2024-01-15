defmodule Queens do
  @type t :: %Queens{black: {integer, integer}, white: {integer, integer}}
  defstruct [:white, :black]

  @doc """
  Creates a new set of Queens
  """
  @spec new(Keyword.t()) :: Queens.t()
  def new(opts \\ []) do
    {white, black} = validate_coords(Keyword.get(opts, :white), Keyword.get(opts, :black))

    %__MODULE__{white: white, black: black}
  end

  defp validate_coords(white, black) do
    validate_negative(white)
    validate_negative(black)

    validate_bounds(white)
    validate_bounds(black)

    validate_same_place(white, black)

    {white, black}
  end

  defp validate_bounds({x, y}) when x > 7 or y > 7 do
    raise ArgumentError
  end

  defp validate_bounds(_), do: :ok

  defp validate_negative({x, y}) when x < 0 or y < 0 do
    raise ArgumentError
  end

  defp validate_negative(_), do: :ok

  defp validate_same_place(x, y) when x == y do
    raise ArgumentError
  end

  defp validate_same_place(_, _), do: :ok

  @doc """
  Gives a string representation of the board with
  white and black queen locations shown
  """
  @spec to_string(Queens.t()) :: String.t()
  def to_string(queens) do
    0..7
    |> Enum.map(fn row ->
      0..7 |> Enum.map(fn col -> draw_square(row, col, queens) end) |> Enum.join(" ")
    end)
    |> Enum.join("\n")
  end

  defp draw_square(row, col, queens) do
    white = queens.white
    black = queens.black

    case {row, col} do
      ^white ->
        "W"

      ^black ->
        "B"

      _ ->
        "_"
    end
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
