defmodule Triangle do
  @type kind :: :equilateral | :isosceles | :scalene

  @doc """
  Return the kind of triangle of a triangle with 'a', 'b' and 'c' as lengths.
  """
  @spec kind(number, number, number) :: {:ok, kind} | {:error, String.t()}
  def kind(a, b, c) when a <= 0 or b <= 0 or c <= 0,
    do: {:error, "all side lengths must be positive"}

  def kind(a, b, c) when a == b and b == c, do: maybe_kind(a, b, c, :equilateral)
  def kind(a, b, c) when a == b or a == c or b == c, do: maybe_kind(a, b, c, :isosceles)
  def kind(a, b, c), do: maybe_kind(a, b, c, :scalene)

  defp maybe_kind(a, b, c, type) do
    violate_triangle_inequality? = Enum.any?([a, b, c], &(a + b + c - &1 < &1))

    if violate_triangle_inequality? do
      {:error, "side lengths violate triangle inequality"}
    else
      {:ok, type}
    end
  end
end
