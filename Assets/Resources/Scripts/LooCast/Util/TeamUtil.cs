using LooCast.Health;
using LooCast.Item;
using System;
using System.Collections.Generic;
using System.Linq.Expressions;
using UnityEngine;

namespace LooCast.Util
{
	public static class TeamUtil
	{
		public static string[] GetEnemyTags(GameObject allyObject)
		{
            IHealth.TeamType team = GetTeam(allyObject.tag);
            if (!Constants.EnemyTagsDictionary.TryGetValue(team, out string[] enemyTags))
            {
                throw new Exception("Invalid Team Type!");
            }
            return enemyTags;
        }
		public static string[] GetEnemyTags(IHealth health)
		{
            if (!Constants.EnemyTagsDictionary.TryGetValue(health.Team, out string[] enemyTags))
            {
                throw new Exception("Invalid Team Type!");
            }
            return enemyTags;
        }
        public static string[] GetEnemyTags(IHealth.TeamType team)
        {
            if (!Constants.EnemyTagsDictionary.TryGetValue(team, out string[] enemyTags))
            {
                throw new Exception("Invalid Team Type!");
            }
            return enemyTags;
        }

		public static IHealth.TeamType GetTeam(string tag)
		{
			switch (tag)
			{
				case "Player":
					return IHealth.TeamType.PlayerAlly;
				case "Ally":
					return IHealth.TeamType.PlayerAlly;
				case "Enemy":
					return IHealth.TeamType.PlayerEnemy;
				default:
					throw new ArgumentException("Unhandled Tag!");
			}
		}

		public static LayerMask GetEnemyLayerMask(string tag)
        {
            switch (tag)
            {
                case "Player":
                    return LayerMask.GetMask("Enemy");
                case "Ally":
                    return LayerMask.GetMask("Enemy");
                case "Enemy":
                    return LayerMask.GetMask("Ally", "Player");
                default:
                    throw new ArgumentException("Unhandled Tag!");
            }
        }
		public static LayerMask GetEnemyLayerMask(IHealth.TeamType team)
		{
			switch (team)
			{
				case IHealth.TeamType.PlayerAlly:
					return LayerMask.GetMask("Enemy");
				case IHealth.TeamType.PlayerEnemy:
                    return LayerMask.GetMask("Ally", "Player");
				default:
                    throw new ArgumentException("Unhandled Team!");
            }
        }
        public static LayerMask GetEnemyLayerMask(GameObject allyObject)
        {
            IHealth.TeamType team = GetTeam(allyObject.tag);
            switch (team)
            {
                case IHealth.TeamType.PlayerAlly:
                    return LayerMask.GetMask("Enemy");
                case IHealth.TeamType.PlayerEnemy:
                    return LayerMask.GetMask("Ally", "Player");
                default:
                    throw new ArgumentException("Unhandled Team!");
            }
        }
    } 
}
