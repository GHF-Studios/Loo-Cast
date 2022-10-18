using LooCast.Item;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Util
{
	public static class TagUtil
	{
		public static string[] GetEnemyTags(GameObject allyObject)
		{
			return Constants.EnemyTagsDictionary.GetValueOrDefault(allyObject.tag);
        }
	} 
}
