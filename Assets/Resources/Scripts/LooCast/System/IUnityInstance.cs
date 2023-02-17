namespace LooCast.System
{
    using global::System.Collections.Generic;
    using LooCast.System.Identification;

    public interface IUnityInstance : IInstance, IUnityInstanceIdentifiable
    {
        public IUnityInstanceType UnityInstanceType { get; }
        public UnityEngine.Object UnityInstanceObject { get; }
        public IUnityInstance ParentUnityInstance { get; }
        public List<IUnityInstance> ChildUnityInstances { get; }
    }
}
