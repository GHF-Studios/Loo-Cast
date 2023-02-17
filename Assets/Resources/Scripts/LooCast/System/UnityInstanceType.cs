﻿using System.Collections.Generic;
using System.Linq;
using CSSystem = System;

namespace LooCast.System
{
    public class UnityInstanceType : Type, IUnityInstanceType
    {
        #region Properties
        public IInstanceType ParentInstanceType => parentUnityInstanceType;
        public IUnityInstanceType ParentUnityInstanceType => parentUnityInstanceType;
        public List<IInstanceType> ChildInstanceTypes => childUnityInstanceTypes.Cast<IInstanceType>().ToList();
        public List<IUnityInstanceType> ChildUnityInstanceTypes => childUnityInstanceTypes;
        #endregion

        #region Fields
        private IUnityInstanceType parentUnityInstanceType;
        private List<IUnityInstanceType> childUnityInstanceTypes;

        public UnityInstanceType(CSSystem.Type cssystemType, Namespace rootNamespace) : base(cssystemType, rootNamespace)
        {
            parentUnityInstanceType = null;
            childUnityInstanceTypes = new List<IUnityInstanceType>();
        }

        public UnityInstanceType(CSSystem.Type systemType, Namespace rootNamespace, UnityInstanceType parentType) : base(systemType, rootNamespace, parentType)
        {
            parentUnityInstanceType = parentType;
            childUnityInstanceTypes = new List<IUnityInstanceType>();
        }
        #endregion

        #region Methods
        public void AddChildUnityInstanceType(UnityInstanceType childUnityInstanceType)
        {
            AddChildType(childUnityInstanceType);
            childUnityInstanceTypes.Add(childUnityInstanceType);
        }
        #endregion
    }
}
