defmodule Wordy do
  @doc """
  Calculate the math problem in the sentence.
  """
  @spec answer(String.t()) :: integer
  def answer(question) do
    ops_and_nums = parse(question)

    Enum.reduce(ops_and_nums, 0, fn
      [op, num], acc ->
        num = String.to_integer(num)

        case op do
          "is" -> num
          "plus" -> acc + num
          "minus" -> acc - num
          "multiplied" -> acc * num
          "divided" -> acc / num
          _ -> raise ArgumentError
        end

      _, _acc ->
        raise ArgumentError
    end)
  end

  defp parse(question) do
    question
    |> String.replace("What ", "")
    |> String.replace("?", "")
    |> String.split()
    |> Enum.reject(&(&1 == "by"))
    |> Enum.chunk_every(2)
  end
end
