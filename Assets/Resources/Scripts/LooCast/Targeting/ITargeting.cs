using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Targeting
{
    using LooCast.Target;

    public interface ITargeting
    {
        List<Target> closestTargets { get; }
        List<Target> furthestTargets { get; }
        List<Target> randomTargets { get; }
        List<Target> randomOnscreenTargets { get; }
        List<Target> randomProximityTargets { get; }
    }
}
