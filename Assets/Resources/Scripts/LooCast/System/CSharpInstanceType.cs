using System.Collections.Generic;
using System.Linq;
using CSSystem = System;

namespace LooCast.System
{
    public class CSharpInstanceType : Type, ICSharpInstanceType
    {
        #region Properties
        public IInstanceType ParentInstanceType => parentCSharpInstanceType;
        public ICSharpInstanceType ParentCSharpInstanceType => parentCSharpInstanceType;
        public List<IInstanceType> ChildInstanceTypes => childCSharpInstanceTypes.Cast<IInstanceType>().ToList();
        public List<ICSharpInstanceType> ChildCSharpInstanceTypes => childCSharpInstanceTypes;
        #endregion

        #region Fields
        protected ICSharpInstanceType parentCSharpInstanceType;
        protected List<ICSharpInstanceType> childCSharpInstanceTypes;

        public CSharpInstanceType(CSSystem.Type cssystemType, Namespace rootNamespace) : base(cssystemType, rootNamespace)
        {
            parentCSharpInstanceType = null;
            childCSharpInstanceTypes = new List<ICSharpInstanceType>();
        }

        public CSharpInstanceType(CSSystem.Type systemType, Namespace rootNamespace, CSharpInstanceType parentType) : base(systemType, rootNamespace, parentType)
        {
            parentCSharpInstanceType = parentType;
            childCSharpInstanceTypes = new List<ICSharpInstanceType>();
        }
        #endregion

        #region Methods
        public void AddChildCSharpInstanceType(CSharpInstanceType childCSharpInstanceType)
        {
            AddChildType(childCSharpInstanceType);
            childCSharpInstanceTypes.Add(childCSharpInstanceType);
        }
        #endregion
    }
}
