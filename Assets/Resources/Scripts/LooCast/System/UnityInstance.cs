using System.Linq;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class UnityInstance : IUnityInstance
    {
        #region Properties
        public IInstanceType InstanceType => unityInstanceType;
        public IUnityInstanceType UnityInstanceType => unityInstanceType;
        public object InstanceObject => unityObject;
        public UnityEngine.Object UnityObject => unityObject;
        public IInstance ParentInstance => parentUnityInstance;
        public IUnityInstance ParentUnityInstance => parentUnityInstance;
        public List<IInstance> ChildInstances => childUnityInstances.Cast<IInstance>().ToList();
        public List<IUnityInstance> ChildUnityInstances => childUnityInstances;
        public IIdentifier Identifier => unityInstanceIdentifier;
        public IInstanceIdentifier InstanceIdentifier => unityInstanceIdentifier;
        public IUnityInstanceIdentifier UnityInstanceIdentifier => unityInstanceIdentifier;
        #endregion

        #region Fields
        private IUnityInstanceIdentifier unityInstanceIdentifier;
        private IUnityInstanceType unityInstanceType;
        private UnityEngine.Object unityObject;
        private IUnityInstance parentUnityInstance;
        private List<IUnityInstance> childUnityInstances;
        #endregion

        #region Constructors
        public UnityInstance(UnityEngine.Object unityObject, IUnityInstanceType unityInstanceType)
        {
            unityInstanceIdentifier = new UnityInstanceIdentifier((IUnityInstanceTypeIdentifier)unityInstanceType.TypeIdentifier);
            this.unityInstanceType = unityInstanceType;
            this.unityObject = unityObject;
            parentUnityInstance = null;
            childUnityInstances = new List<IUnityInstance>();
        }

        public UnityInstance(UnityEngine.Object unityObject, IUnityInstanceType unityInstanceType, IUnityInstance parentUnityInstance)
        {
            unityInstanceIdentifier = new UnityInstanceIdentifier((IUnityInstanceTypeIdentifier)unityInstanceType.TypeIdentifier);
            this.unityInstanceType = unityInstanceType;
            this.unityObject = unityObject;
            this.parentUnityInstance = parentUnityInstance;
            childUnityInstances = new List<IUnityInstance>();
        }
        #endregion

        #region Methods
        public void AddChildUnityInstance(IUnityInstance childUnityInstance)
        {
            childUnityInstances.Add(childUnityInstance);
        }
        #endregion
    }
}
