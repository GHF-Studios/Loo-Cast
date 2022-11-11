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
            return GetEnemyTags(team);
        }
		public static string[] GetEnemyTags(IHealth health)
		{
            return GetEnemyTags(health.Team);
        }
        public static string[] GetEnemyTags(IHealth.TeamType team)
        {
            switch (team)
            {
                case IHealth.TeamType.PlayerAlly:
                    return new string[] { "Enemy", "EnemyStation" };
                case IHealth.TeamType.PlayerEnemy:
                    return new string[] { "Ally", "AllyStation", "Player" };
                default:
                    throw new ArgumentException($"Unhandled Team '{Enum.GetName(typeof(IHealth.TeamType), team)}'!");
            }
        }

		public static IHealth.TeamType GetTeam(string tag)
		{
			switch (tag)
			{
				case "Player":
					return IHealth.TeamType.PlayerAlly;
				case "Ally":
					return IHealth.TeamType.PlayerAlly;
                case "AllyStation":
                    return IHealth.TeamType.PlayerAlly;
                case "Enemy":
					return IHealth.TeamType.PlayerEnemy;
                case "EnemyStation":
                    return IHealth.TeamType.PlayerEnemy;
                default:
					throw new ArgumentException($"Unhandled Tag '{tag}'!");
			}
		}

		public static LayerMask GetEnemyLayerMask(string tag)
        {
            IHealth.TeamType team = GetTeam(tag);
            return GetEnemyLayerMask(team);
        }

        public static LayerMask GetEnemyLayerMask(GameObject allyObject)
        {
            IHealth.TeamType team = GetTeam(allyObject.tag);
            return GetEnemyLayerMask(team);
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
                    throw new ArgumentException($"Unhandled Team '{Enum.GetName(typeof(IHealth.TeamType), team)}'!");
            }
        }
    } 
}
