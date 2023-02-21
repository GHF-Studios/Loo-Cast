using System;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System
{
    using Identification;

    public class CSharpInstance : ICSharpInstance
    {
        #region Properties
        public IInstanceType InstanceType => csharpInstanceType;
        public ICSharpInstanceType CSharpInstanceType => csharpInstanceType;
        public object InstanceObject => csharpInstanceObject;
        public object CSharpInstanceObject => csharpInstanceObject;
        public IInstance ParentInstance => parentCSharpInstance;
        public ICSharpInstance ParentCSharpInstance => parentCSharpInstance;
        public List<IInstance> ChildInstances => childCSharpInstances.Cast<IInstance>().ToList();
        public List<ICSharpInstance> ChildCSharpInstances => childCSharpInstances;
        public ICSharpInstanceIdentifier CSharpInstanceIdentifier => csharpInstanceIdentifier;
        public IInstanceIdentifier InstanceIdentifier => csharpInstanceIdentifier;
        public IIdentifier Identifier => csharpInstanceIdentifier;
        #endregion

        #region Fields
        protected ICSharpInstanceIdentifier csharpInstanceIdentifier;
        protected ICSharpInstanceType csharpInstanceType;
        protected object csharpInstanceObject;
        protected ICSharpInstance parentCSharpInstance;
        protected List<ICSharpInstance> childCSharpInstances;
        #endregion

        #region Constructors
        public CSharpInstance(object csharpInstanceObject, ICSharpInstanceType csharpInstanceType)
        {
            csharpInstanceIdentifier = new CSharpInstanceIdentifier((ICSharpInstanceTypeIdentifier)csharpInstanceType.Identifier);
            this.csharpInstanceType = csharpInstanceType;
            this.csharpInstanceObject = csharpInstanceObject;
            parentCSharpInstance = null;
            childCSharpInstances = new List<ICSharpInstance>();
        }

        public CSharpInstance(object csharpInstanceObject, ICSharpInstanceType csharpInstanceType, ICSharpInstance parentCSharpInstance)
        {
            csharpInstanceIdentifier = new CSharpInstanceIdentifier((ICSharpInstanceTypeIdentifier)csharpInstanceType.Identifier);
            this.parentCSharpInstance = parentCSharpInstance;
            childCSharpInstances = new List<ICSharpInstance>();
        }
        #endregion

        #region Methods
        public void AddChildCSharpInstance(ICSharpInstance childCSharpInstance)
        {
            childCSharpInstances.Add(childCSharpInstance);
        }
        #endregion
    }
}
