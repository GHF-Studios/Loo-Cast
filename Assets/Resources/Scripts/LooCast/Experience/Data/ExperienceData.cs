using UnityEngine;

namespace LooCast.Experience.Data
{
    using LooCast.Data;

    public abstract class ExperienceData : ScriptableObject
    {
        public FloatReference InitialExperience;
        public FloatReference InitialLevelExperienceMax;
        public IntReference InitialLevel;
    } 
}