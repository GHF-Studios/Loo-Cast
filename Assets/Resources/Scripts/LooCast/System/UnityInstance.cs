using CSSystem = System;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;

    public class UnityInstance : IUnityInstance
    {
        #region Properties
        public IInstanceType InstanceType => unityInstanceType;
        public IUnityInstanceType UnityInstanceType => unityInstanceType;
        public object InstanceObject => unityInstanceObject;
        public UnityEngine.Object UnityInstanceObject => unityInstanceObject;
        public IInstance ParentInstance => parentUnityInstance;
        public IUnityInstance ParentUnityInstance => parentUnityInstance;
        public List<IInstance> ChildInstances => childUnityInstances.Cast<IInstance>().ToList();
        public List<IUnityInstance> ChildUnityInstances => childUnityInstances;
        public IUnityInstanceIdentifier UnityInstanceIdentifier => unityInstanceIdentifier;
        public IInstanceIdentifier InstanceIdentifier => unityInstanceIdentifier;
        public IIdentifier Identifier => unityInstanceIdentifier;
        #endregion

        #region Fields
        private IUnityInstanceIdentifier unityInstanceIdentifier;
        private IUnityInstanceType unityInstanceType;
        private UnityEngine.Object unityInstanceObject;
        private IUnityInstance parentUnityInstance;
        private List<IUnityInstance> childUnityInstances;
        #endregion

        #region Constructors
        public UnityInstance(UnityEngine.Object unityInstanceObject, IUnityInstanceType unityInstanceType)
        {
            unityInstanceIdentifier = new UnityInstanceIdentifier((IUnityInstanceTypeIdentifier)unityInstanceType.TypeIdentifier);
            this.unityInstanceType = unityInstanceType;
            this.unityInstanceObject = unityInstanceObject;
            parentUnityInstance = null;
            childUnityInstances = new List<IUnityInstance>();
        }

        public UnityInstance(UnityEngine.Object unityInstanceObject, IUnityInstanceType unityInstanceType, IUnityInstance parentUnityInstance)
        {
            unityInstanceIdentifier = new UnityInstanceIdentifier((IUnityInstanceTypeIdentifier)unityInstanceType.Identifier);
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
