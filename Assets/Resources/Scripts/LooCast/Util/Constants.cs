using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Util
{
    public static class Constants
    {
        public const float InertialCoefficient = 50.0f;
        public static readonly Dictionary<string, string[]> EnemyTagsDictionary = new Dictionary<string, string[]>()
        {
            {"Player", new string[] { "Enemy", "EnemyStation" } },
            {"Ally", new string[] { "Enemy", "EnemyStation" } },
            {"AllyStation", new string[] { "Enemy", "EnemyStation" } },
            {"Enemy", new string[] { "Player", "Ally", "AllyStation" } },
            {"EnemyStation", new string[] { "Player", "Ally", "AllyStation" } },
        };
    } 
}
