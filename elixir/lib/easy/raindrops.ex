defmodule Raindrops do
  @doc """
  Returns a string based on raindrop factors.

  - If the number contains 3 as a prime factor, output 'Pling'.
  - If the number contains 5 as a prime factor, output 'Plang'.
  - If the number contains 7 as a prime factor, output 'Plong'.
  - If the number does not contain 3, 5, or 7 as a prime factor,
    just pass the number's digits straight through.
  """
  @spec convert(pos_integer) :: String.t()
  def convert(number) do
    string =
      maybe_add_pling("", number)
      |> maybe_add_plang(number)
      |> maybe_add_plong(number)

    if string == "" do
      Integer.to_string(number)
    else
      string
    end
  end

  defp maybe_add_pling(acc, number) do
    if rem(number, 3) == 0 do
      "#{acc}Pling"
    else
      acc
    end
  end

  defp maybe_add_plang(acc, number) do
    if rem(number, 5) == 0 do
      "#{acc}Plang"
    else
      acc
    end
  end

  defp maybe_add_plong(acc, number) do
    if rem(number, 7) == 0 do
      "#{acc}Plong"
    else
      acc
    end
  end
end
