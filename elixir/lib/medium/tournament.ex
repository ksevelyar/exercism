defmodule Tournament do
  @team %{played: 0, won: 0, drawn: 0, points: 0, lost: 0}
  @header "Team                           | MP |  W |  D |  L |  P"
  @team_padding_length 31
  @points_padding_length 2

  @doc """
  Given `input` lines representing two teams and whether the first of them won,
  lost, or reached a draw, separated by semicolons, calculate the statistics
  for each team's number of games played, won, drawn, lost, and total points
  for the season, and return a nicely-formatted string table.

  A win earns a team 3 points, a draw earns 1 point, and a loss earns nothing.

  Order the outcome by most total points for the season, and settle ties by
  listing the teams in alphabetical order.
  """
  @spec tally(input :: list(String.t())) :: String.t()
  def tally(input) do
    input
    |> count_scores()
    |> render_teams()
  end

  defp count_scores(matches) do
    Enum.reduce(matches, %{}, fn line, acc ->
      case String.split(line, ";") do
        [team1, team2, "win"] -> add_win(acc, team1, team2)
        [team1, team2, "loss"] -> add_win(acc, team2, team1)
        [team1, team2, "draw"] -> add_draw(acc, team1, team2)
        _ -> acc
      end
    end)
  end

  defp add_win(teams, team1_name, team2_name) do
    team1 = teams[team1_name] || @team
    team2 = teams[team2_name] || @team

    team1 = %{
      team1
      | played: team1.played + 1,
        won: team1.won + 1,
        points: team1.points + 3
    }

    team2 = %{
      team2
      | played: team2.played + 1,
        lost: team2.lost + 1
    }

    teams
    |> Map.put(team1_name, team1)
    |> Map.put(team2_name, team2)
  end

  defp add_draw(teams, team1_name, team2_name) do
    team1 = teams[team1_name] || @team
    team2 = teams[team2_name] || @team

    team1 = %{
      team1
      | played: team1.played + 1,
        drawn: team1.drawn + 1,
        points: team1.points + 1
    }

    team2 = %{
      team2
      | played: team2.played + 1,
        drawn: team2.drawn + 1,
        points: team2.points + 1
    }

    teams
    |> Map.put(team1_name, team1)
    |> Map.put(team2_name, team2)
  end

  defp render_teams(teams) do
    teams =
      teams
      |> Enum.sort_by(fn {name, %{points: points}} -> {-points, name} end, :asc)
      |> Enum.map(fn {name, t} ->
        name = String.pad_trailing(name, @team_padding_length)
        points = String.pad_leading(Integer.to_string(t.points), @points_padding_length)

        "#{name}|  #{t.played} |  #{t.won} |  #{t.drawn} |  #{t.lost} | #{points}"
      end)

    Enum.join([@header | teams], "\n")
  end
end
