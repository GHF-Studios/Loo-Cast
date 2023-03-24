using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public interface IUnityInstance : IInstance, IUnityInstanceIdentifiable
    {
        #region Properties
        public IUnityInstanceType UnityInstanceType { get; }
        public UnityEngine.Object UnityObject { get; }
        public IUnityInstance ParentUnityInstance { get; }
        public List<IUnityInstance> ChildUnityInstances { get; }
        #endregion
    }
}
