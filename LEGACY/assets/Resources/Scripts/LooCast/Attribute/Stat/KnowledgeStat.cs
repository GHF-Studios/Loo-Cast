using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "KnowledgeStat", menuName = "Data/Attribute/Stat/KnowledgeStat", order = 0)]
    public class KnowledgeStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Knowledge";
            }
        }
        public float LevelExperienceMaxMultiplier
        {
            get
            {
                return 1.75f - Level.Value * 0.01f;
            }
        }
    } 
}
