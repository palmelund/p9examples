using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BossControl : MonoBehaviour, IEnemy
{

	public float Speed;
	public float moveTime;
	public float MaxHeight;
	private float _spawnTime;
	private bool _movingUp = false;
	public int ShieldCount = 3;

	public float HomingInterval = 3;
	private float _homingSpawnTime = 0;

	public GameObject HomingMissile;

	private int _score = 200;
	// Use this for initialization
	void Start ()
	{
		_spawnTime = Time.time;
	}
	public int ScoreReward
	{
		get { return _score; }
	}
	
	// Update is called once per frame
	void Update () {
		if (Time.time < _spawnTime+moveTime)
		{
			transform.position += Vector3.left*Time.deltaTime*Speed;
		}
		else
		{
			if (_movingUp)
			{
				transform.position += Vector3.up*Time.deltaTime*Speed;
			}
			else
			{
				transform.position += Vector3.down*Time.deltaTime*Speed;
			}
		}

		if (Math.Abs(transform.position.y) > MaxHeight)
		{
			_movingUp =! _movingUp;
		}

		if (Time.time >= _homingSpawnTime + HomingInterval)
		{
			Instantiate(HomingMissile, transform.position, Quaternion.identity);
			_homingSpawnTime = Time.time;
		}
	}

}
