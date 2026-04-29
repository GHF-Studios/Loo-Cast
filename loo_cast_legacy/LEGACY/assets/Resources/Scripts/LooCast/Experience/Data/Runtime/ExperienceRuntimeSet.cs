using UnityEngine;

namespace LooCast.Experience.Data.Runtime
{
    using LooCast.Data.Runtime;
    using LooCast.Experience;

    [CreateAssetMenu(fileName = "ExampleComponentRuntimeSet", menuName = "Data/Runtime/ExampleComponentRuntimeSet", order = 0)]
    public class ExperienceRuntimeSet<T> : RuntimeSet<IExperience>
    {

    } 
}