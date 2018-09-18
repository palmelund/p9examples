using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class EnemyBulletControl : MonoBehaviour
{
	public float Speed;

	void Update ()
	{
		transform.position += Vector3.left * Time.deltaTime * Speed;
	}

	private void OnCollisionEnter(Collision other)
	{
		if (other.gameObject.tag.Equals("Player"))  
		{
			GameObject player = GameObject.FindGameObjectWithTag("Player");
			player.GetComponent<PlayerControl>().Health--;
			Destroy(gameObject);
		}
	}
}
