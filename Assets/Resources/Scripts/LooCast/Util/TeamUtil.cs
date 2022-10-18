using LooCast.Health;
using LooCast.Item;
using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Util
{
	public static class TeamUtil
	{
		public static string[] GetEnemyTags(GameObject allyObject)
		{
			return Constants.EnemyTagsDictionary.GetValueOrDefault(allyObject.tag);
        }

		public static string[] GetEnemyTags(IHealth health)
		{
			switch (health.Team)
			{
				case IHealth.TeamType.PlayerAlly:
					return Constants.EnemyTagsDictionary.GetValueOrDefault("Ally");
				case IHealth.TeamType.PlayerEnemy:
                    return Constants.EnemyTagsDictionary.GetValueOrDefault("Enemy");
				default:
					throw new ArgumentException("Unhandled TeamType!");
			}
		}

        public static string[] GetEnemyTags(IHealth.TeamType team)
        {
            switch (team)
            {
                case IHealth.TeamType.PlayerAlly:
                    return Constants.EnemyTagsDictionary.GetValueOrDefault("Ally");
                case IHealth.TeamType.PlayerEnemy:
                    return Constants.EnemyTagsDictionary.GetValueOrDefault("Enemy");
                default:
                    throw new ArgumentException("Unhandled Team!");
            }
        }

		public static IHealth.TeamType GetTeamFromTag(string tag)
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
    } 
}
