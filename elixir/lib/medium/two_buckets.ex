defmodule TwoBuckets do
  defstruct [:bucket_one, :bucket_two, :moves]
  @type t :: %TwoBuckets{bucket_one: integer, bucket_two: integer, moves: integer}

  @doc """
  Find the quickest way to fill a bucket with some amount of water from two buckets of specific sizes.
  """
  @spec measure(
          size_one :: integer,
          size_two :: integer,
          goal :: integer,
          start_bucket :: :one | :two
        ) :: {:ok, TwoBuckets.t()} | {:error, :impossible}
  def measure(size_one, size_two, goal, :one) when size_one < size_two do
    pour_from_small_to_big(size_one, size_two, goal, size_one, 0, 1)
  end

  def measure(size_one, size_two, goal, :one) do
    pour_from_big_to_small(size_one, size_two, goal, size_one, 0, 1)
  end

  def measure(size_one, size_two, goal, :two) when size_two < size_one do
    pour_from_small_to_big(size_one, size_two, goal, 0, size_two, 1)
  end

  def measure(size_one, size_two, goal, :two) do
    pour_from_big_to_small(size_one, size_two, goal, 0, size_two, 1)
  end

  defp pour_from_big_to_small(_, _, goal, amount_one, amount_two, moves)
       when goal in [amount_one, amount_two] do
    {:ok, %TwoBuckets{bucket_one: amount_one, bucket_two: amount_two, moves: moves}}
  end

  defp pour_from_big_to_small(size_one, size_two, goal, amount_one, amount_two, moves) do
    case {amount_one, amount_two} do
      {0, b} when b > size_one ->
        pour_from_big_to_small(size_one, size_two, goal, size_one, b - size_one, moves + 1)

      {0, b} ->
        pour_from_big_to_small(size_one, size_two, goal, b, 0, moves + 1)

      {a, 0} ->
        pour_from_big_to_small(size_one, size_two, goal, a, size_two, moves + 1)

      {a, b} when a < size_one ->
        pour_from_big_to_small(size_one, size_two, goal, size_one, b - (size_one - a), moves + 1)

      {a, b} when a == size_one ->
        pour_from_big_to_small(size_one, size_two, goal, 0, b, moves + 1)

      _ ->
        {:error, :impossible}
    end
  end

  defp pour_from_small_to_big(_, _, goal, amount_one, amount_two, moves)
       when goal in [amount_one, amount_two] do
    {:ok, %TwoBuckets{bucket_one: amount_one, bucket_two: amount_two, moves: moves}}
  end

  defp pour_from_small_to_big(size_one, size_two, goal, amount_one, amount_two, moves) do
    case {amount_one, amount_two} do
      {^size_one, ^size_two} ->
        {:error, :impossible}

      {0, b} ->
        pour_from_small_to_big(size_one, size_two, goal, size_one, b, moves + 1)

      {a, 0} when size_two == goal ->
        pour_from_small_to_big(size_one, size_two, goal, a, size_two, moves + 1)

      {a, b} when a + b <= size_two ->
        pour_from_small_to_big(size_one, size_two, goal, 0, a + b, moves + 1)

      {a, b} when a + b - goal == size_two ->
        pour_from_small_to_big(size_one, size_two, goal, goal, a + b - goal, moves + 1)

      {a, b} when b < size_two ->
        pour_from_small_to_big(size_one, size_two, goal, a - (size_two - b), size_two, moves + 1)

      {a, b} when b == size_two ->
        pour_from_small_to_big(size_one, size_two, goal, a, 0, moves + 1)
    end
  end
end
