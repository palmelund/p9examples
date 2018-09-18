using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class EnemyControl : MonoBehaviour, IEnemy
{
	public float Speed;
	private int _scoreReward = 10;

	private bool _hasShot = false;

	public GameObject EnemyBulletControl;
	
	public int ScoreReward
	{
		get { return _scoreReward;} 
	}

	private static Transform _playerControl;
	
	void Start()
	{
		if (_playerControl == null)
		{
			var tmp = GameObject.FindGameObjectWithTag("Player");
			if (tmp)
			{
				_playerControl = tmp.transform;
			}
		}
	}
	
	// Update is called once per frame
	void Update ()
	{
		if (!_playerControl)
		{
			return;
		}
		
		transform.position += Vector3.left * Time.deltaTime * Speed;

		if (Math.Abs(transform.position.y - _playerControl.position.y) < 1f && !_hasShot)
		{
			_hasShot = true;
			Instantiate(EnemyBulletControl, transform.position, Quaternion.identity );
		}
	}

}
