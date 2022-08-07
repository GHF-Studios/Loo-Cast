using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "KnowledgeStat", menuName = "Data/Attribute/Stat/KnowledgeStat", order = 0)]
    public class KnowledgeStat : Stat
    {
        public FloatComputedVariable LevelExperienceMaxMultiplier;
    } 
}
