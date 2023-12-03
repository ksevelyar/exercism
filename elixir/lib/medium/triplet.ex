defmodule Triplet do
  @doc """
  Calculates sum of a given triplet of integers.
  """
  @spec sum([non_neg_integer]) :: non_neg_integer
  def sum([a, b, c]) do
    a + b + c
  end

  @doc """
  Calculates product of a given triplet of integers.
  """
  @spec product([non_neg_integer]) :: non_neg_integer
  def product([a, b, c]) do
    a * b * c
  end

  @doc """
  Determines if a given triplet is pythagorean. That is, do the squares of a and b add up to the square of c?
  """
  @spec pythagorean?([non_neg_integer]) :: boolean
  def pythagorean?([a, b, c]) do
    a < b && b < c && a * a + b * b == c * c
  end

  @doc """
  Generates a list of pythagorean triplets whose values add up to a given sum.
  """
  @spec generate(non_neg_integer) :: [list(non_neg_integer)]
  def generate(sum) do
    2..sum
    |> Enum.flat_map(fn m ->
      find_triplets(m, 1, sum, [])
    end)
    |> dbg
  end

  def find_triplets(m, n, limit, result) do
    a = m * m - n * n
    b = 2 * m * n
    c = m * m + n * n

    if n > m do
      result
    else
      result =
        if a + b + c == limit do
          [[a, b, c] | result]
        else
          result
        end

      find_triplets(m, n + 1, limit, result)
    end
  end
end
