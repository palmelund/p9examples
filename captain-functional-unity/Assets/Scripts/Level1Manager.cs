using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Level1Manager : MonoBehaviour
{
	private int _score = 0;
	private UIManager _uiManager;
	public int Score
	{
		get { return _score;}
		set
		{
			_score = value;
			_uiManager.UpdateScore(_score);
		}
	}
	// Use this for initialization
	void Awake ()
	{
		_uiManager = FindObjectOfType<UIManager>();
	}
	
	// Update is called once per frame
	void Update () {
		
	}
}
