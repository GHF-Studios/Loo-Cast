using LooCast.Health;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Util
{
    public static class Constants
    {
        public const float InertialCoefficient = 50.0f;
        public static readonly Dictionary<IHealth.TeamType, string[]> EnemyTagsDictionary = new Dictionary<IHealth.TeamType, string[]>()
        {
            {IHealth.TeamType.PlayerAlly, new string[] { "Enemy" } },
            {IHealth.TeamType.PlayerEnemy, new string[] { "Ally", "Player" } },
        };
    } 
}
