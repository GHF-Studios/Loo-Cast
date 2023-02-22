using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IUnityInstance : IInstance, IUnityInstanceIdentifiable
    {
        #region Properties
        public IUnityInstanceType UnityInstanceType { get; }
        public UnityEngine.Object UnityInstanceObject { get; }
        public IUnityInstance ParentUnityInstance { get; }
        public List<IUnityInstance> ChildUnityInstances { get; }
        #endregion
    }
}
