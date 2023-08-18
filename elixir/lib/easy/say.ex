defmodule Say do
  @upper_limit 999_999_999_999

  @doc """
  Translate a positive integer into English.
  """
  @spec in_english(integer) :: {atom, String.t()}
  def in_english(0), do: {:ok, "zero"}

  def in_english(number) when number < 0 or number > @upper_limit,
    do: {:error, "number is out of range"}

  def in_english(number) do
    thousands =
      number |> Integer.digits() |> Enum.reverse() |> Enum.chunk_every(3) |> Enum.reverse()

    len = Enum.count(thousands)

    {:ok,
     thousands
     |> Enum.with_index()
     |> Enum.map(fn {digits, index} ->
       digits = Enum.reverse(digits)
       power = 1000 ** (len - index - 1)

       case digits do
         [0, 0, 0] -> ""
         [a, 0, 0] -> "#{say_number([a])}#{say_thousands(power * 100)}"
         [0, 0, c] -> "#{say_number([c])}#{say_thousands(power)}"
         [0, b, c] -> "#{say_number([b, c])}#{say_thousands(power)}"
         [a, b, c] -> "#{say_number([a])} hundred #{say_number([b, c])}#{say_thousands(power)}"
         _ -> "#{say_number(digits)}#{say_thousands(power)}"
       end
     end)
     |> Enum.reject(&(&1 == ""))
     |> Enum.join(" ")}
  end

  defp say_number([tens, ones]) when tens in 2..9 do
    tens =
      case tens do
        2 -> "twenty"
        3 -> "thirty"
        4 -> "forty"
        5 -> "fifty"
        6 -> "sixty"
        7 -> "seventy"
        8 -> "eighty"
        9 -> "ninety"
      end

    [tens, say_number([ones])]
    |> Enum.reject(&is_nil/1)
    |> Enum.join("-")
  end

  defp say_number(digits) do
    case digits do
      [1] -> "one"
      [2] -> "two"
      [3] -> "three"
      [4] -> "four"
      [5] -> "five"
      [6] -> "six"
      [7] -> "seven"
      [8] -> "eight"
      [9] -> "nine"
      [1, 0] -> "ten"
      [1, 1] -> "eleven"
      [1, 2] -> "twelve"
      [1, 3] -> "thirteen"
      [1, 4] -> "fourteen"
      [1, 5] -> "fifteen"
      [1, 6] -> "sixteen"
      [1, 7] -> "seventeen"
      [1, 8] -> "eighteen"
      [1, 9] -> "nineteen"
      [0, 0] -> nil
      [0, n] -> say_number(n)
      _ -> nil
    end
  end

  defp say_thousands(number) do
    case number do
      100 -> " hundred"
      1_000 -> " thousand"
      1_000_000 -> " million"
      1_000_000_000 -> " billion"
      _ -> nil
    end
  end
end
