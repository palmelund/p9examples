using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;
using UnityEngine;
using UnityEngine.UI;

public class HighscoreController : MonoBehaviour
{
    public Text HighScoreText;
    public Text PlayerName;

    public void AddScore()
    {
        var scoreboard = LoadScoreBoard();

        scoreboard.Add(new Tuple<string, int>(PlayerName.text, GetComponent<Level1Manager>().Score));

        SaveScoreBoard(scoreboard);
    }

    public void Build()
    {
        var output = string.Empty;

        var scoreboard = LoadScoreBoard();

        foreach (var score in scoreboard.OrderByDescending(tuple => tuple.Item2).Take(10))
        {
            output += $"{score.Item1} - {score.Item2}\n";
        }

        HighScoreText.text = output;
    }

    private List<Tuple<string, int>> LoadScoreBoard()
    {
        var scoreboardStr = PlayerPrefs.GetString("scoreboard");
        var scoreboard = Newtonsoft.Json.JsonConvert.DeserializeObject<List<Tuple<string, int>>>(scoreboardStr) ??
                                new List<Tuple<string, int>>();

        return scoreboard;
    }

    private void SaveScoreBoard(List<Tuple<string, int>> scoreboard)
    {
        var res = JsonConvert.SerializeObject(scoreboard);
        PlayerPrefs.SetString("scoreboard", res);
    }
}