using System.Collections;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

public class PlayerBulletControl : MonoBehaviour
{
	public float Speed;

	private static Level1Manager _level1Manager;
	// Use this for initialization
	void Start () {
		if (_level1Manager == null)
		{
			_level1Manager = FindObjectOfType<Level1Manager>();
			_level1Manager.Score = 0;
		}
	}
	
	// Update is called once per frame
	void Update ()
	{
		transform.position += Vector3.right * Time.deltaTime * Speed;
	}

	private void OnCollisionEnter(Collision other)
	{
		Debug.Log(other.gameObject.name);
		Debug.Log(other.gameObject.tag);
		if (other.gameObject.tag.Equals("Enemy"))
		{
			Destroy(other.gameObject);
			Destroy(gameObject);
			_level1Manager.Score += other.gameObject.GetComponent<IEnemy>().ScoreReward;

		}
		else if (other.gameObject.tag.Equals("Boss") && other.gameObject.GetComponent<BossControl>().ShieldCount == 0)  
		{
			GameObject player = GameObject.FindGameObjectWithTag("Player");
			_level1Manager.Score += other.gameObject.GetComponent<IEnemy>().ScoreReward;
			Destroy(other.gameObject);
			player.GetComponent<PlayerControl>().Event.Invoke();
			Destroy(player);
			Destroy(gameObject);
		}
		else if (other.gameObject.tag.Equals("HomingMissle"))
		{
			Destroy(other.gameObject);
			Destroy(gameObject);
		}
	}
}
