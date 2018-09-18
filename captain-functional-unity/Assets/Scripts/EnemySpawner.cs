using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class EnemySpawner : MonoBehaviour
{
	public float WaitTimer = 1;
	private int _spawnCount = 0;
	public int Spawns = 50;
	public List<GameObject> Enemies;
	public GameObject Boss;
	private bool _bossSpawned = false;
	private Vector3 _v3PosButtom,_v3PosTop;
	// Use this for initialization
	void Start ()
	{
		_v3PosButtom = Camera.main.ViewportToWorldPoint(new Vector3(1.1f, 0.9f, 150.0f));
		_v3PosTop = Camera.main.ViewportToWorldPoint(new Vector3(1.1f, 0.1f, 150.0f));
		StartCoroutine(Spawn());
	}

	void Update()
	{
		if (_spawnCount >= Spawns && !_bossSpawned)
		{
			Instantiate(Boss,
				new Vector3(_v3PosButtom.x, 0, _v3PosButtom.z), Quaternion.identity);
			_bossSpawned = true;
		}
	}

	IEnumerator Spawn()
	{
		while (true)
		{
			
			int enemySelect = Random.Range(0, Enemies.Count);
			Instantiate(Enemies[enemySelect],
				new Vector3(_v3PosButtom.x, Random.Range(_v3PosButtom.y, _v3PosTop.y), _v3PosButtom.z), Quaternion.identity);
			_spawnCount++;
			yield return new WaitForSeconds(WaitTimer);
		}
	}
}
