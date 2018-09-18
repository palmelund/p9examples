using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class BossShield : MonoBehaviour, IEnemy
{
	public float length = 0;
	public float Speed = 0;
	
	private int _score = 0;
	private float _position;
	public float StartPosition;
	private BossControl _bossControl;
	public int ScoreReward
	{
		get { return _score; }
	}

	private float _circumference; 
	// Use this for initialization
	void Start ()
	{
		_circumference = (float)(Math.PI * 2 * length);
		_bossControl = transform.parent.GetComponent<BossControl>();
		_position = (Mathf.PI * 2) * (StartPosition / 3);
	}
	
	// Update is called once per frame
	void Update ()
	{
		_position += (Mathf.PI*2)*(Time.deltaTime * Speed) / _circumference;
		float posx = Mathf.Cos(_position)*length;
		float posy = Mathf.Sin(_position)*length;
		transform.localPosition = new Vector3(posx,posy, 0);
	}

	private void OnDestroy()
	{
		_bossControl.ShieldCount--;
	}
}
