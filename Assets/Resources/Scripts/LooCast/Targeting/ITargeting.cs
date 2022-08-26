using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Targeting
{
    using LooCast.Target;

    public interface ITargeting
    {
        #region Properties
        List<Target> ClosestTargets { get; }
        List<Target> FurthestTargets { get; }
        List<Target> RandomTargets { get; }
        List<Target> RandomOnscreenTargets { get; }
        List<Target> RandomProximityTargets { get; }
        #endregion
    }
}
