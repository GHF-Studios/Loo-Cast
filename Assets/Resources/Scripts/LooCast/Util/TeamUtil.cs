using LooCast.Health;
using LooCast.Item;
using System;
using System.Collections.Generic;
using System.Linq;
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
                case IHealth.TeamType.Ally:
                    return new string[] { "Enemy" };
                case IHealth.TeamType.Enemy:
                    return new string[] { "Ally", "Player" };
                case IHealth.TeamType.Neutral:
                    throw new InvalidOperationException($"Invalid Team 'Neutral'! The 'Neutral' Team has no Enemies, nor Allies.");
                default:
                    throw new ArgumentException($"Unhandled Team '{Enum.GetName(typeof(IHealth.TeamType), team)}'!");
            }
        }

		public static IHealth.TeamType GetTeam(string tag)
		{
			switch (tag)
			{
				case "Player":
					return IHealth.TeamType.Ally;
				case "Ally":
					return IHealth.TeamType.Ally;
                case "Enemy":
					return IHealth.TeamType.Enemy;
                case "Neutral":
                    return IHealth.TeamType.Neutral;
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
				case IHealth.TeamType.Ally:
					return LayerMask.GetMask("Enemy");
				case IHealth.TeamType.Enemy:
                    return LayerMask.GetMask("Ally", "Player");
                case IHealth.TeamType.Neutral:
                    throw new InvalidOperationException($"Invalid Team 'Neutral'! The 'Neutral' Team has no Enemies, nor Allies.");
				default:
                    throw new ArgumentException($"Unhandled Team '{Enum.GetName(typeof(IHealth.TeamType), team)}'!");
            }
        }
    } 
}
