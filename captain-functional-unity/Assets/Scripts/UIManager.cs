using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class UIManager : MonoBehaviour
{
	public Text PlayerHealth;
	public Text UIScore;
	
	public void ExitGame()
	{
		Application.Quit();
	}

	public void ChangeScene(int sceneNumber)
	{
		SceneManager.LoadScene((sceneNumber));
	}

	public void UpdateHealth(int health)
	{
		PlayerHealth.text = "Health: " + health;
	}

	public void UpdateScore(int score)
	{
		UIScore.text = "Score: " + score;
	}
}
