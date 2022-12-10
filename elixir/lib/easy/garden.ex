defmodule Garden do
  @student_names [
    :alice,
    :bob,
    :charlie,
    :david,
    :eve,
    :fred,
    :ginny,
    :harriet,
    :ileana,
    :joseph,
    :kincaid,
    :larry
  ]

  @plants %{
    "C" => :clover,
    "G" => :grass,
    "R" => :radishes,
    "V" => :violets
  }

  @doc """
    Accepts a string representing the arrangement of cups on a windowsill and a
    list with names of students in the class. The student names list does not
    have to be in alphabetical order.

    It decodes that string into the various gardens for each student and returns
    that information in a map.
  """
  @spec info(String.t(), list) :: map
  def info(info_string, student_names \\ @student_names) do
    plants =
      String.split(info_string)
      |> Enum.map(&(String.split(&1, "", trim: true) |> Enum.chunk_every(2)))

    students = student_names |> Enum.sort() |> Enum.with_index()

    for {student, index} <- students, into: %{} do
      {student, plants_by_index(plants, index)}
    end
  end

  defp plants_by_index(plants, index) do
    case Enum.map(plants, &Enum.at(&1, index)) do
      [nil, nil] -> {}
      [[a, b], [c, d]] -> Enum.map([a, b, c, d], &@plants[&1]) |> List.to_tuple()
    end
  end
end
