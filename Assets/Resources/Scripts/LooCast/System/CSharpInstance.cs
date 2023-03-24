using System.Linq;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class CSharpInstance : ICSharpInstance
    {
        #region Properties
        public IInstanceType InstanceType => csharpInstanceType;
        public ICSharpInstanceType CSharpInstanceType => csharpInstanceType;
        public object InstanceObject => systemObject;
        public object SystemObject => systemObject;
        public IInstance ParentInstance => parentCSharpInstance;
        public ICSharpInstance ParentCSharpInstance => parentCSharpInstance;
        public List<IInstance> ChildInstances => childCSharpInstances.Cast<IInstance>().ToList();
        public List<ICSharpInstance> ChildCSharpInstances => childCSharpInstances;
        public ICSharpInstanceIdentifier CSharpInstanceIdentifier => csharpInstanceIdentifier;
        public IInstanceIdentifier InstanceIdentifier => csharpInstanceIdentifier;
        public IIdentifier Identifier => csharpInstanceIdentifier;
        #endregion

        #region Fields
        private ICSharpInstanceIdentifier csharpInstanceIdentifier;
        private ICSharpInstanceType csharpInstanceType;
        private object systemObject;
        private ICSharpInstance parentCSharpInstance;
        private List<ICSharpInstance> childCSharpInstances;
        #endregion

        #region Constructors
        public CSharpInstance(object systemObject, ICSharpInstanceType csharpInstanceType)
        {
            csharpInstanceIdentifier = new CSharpInstanceIdentifier((ICSharpInstanceTypeIdentifier)csharpInstanceType.Identifier);
            this.csharpInstanceType = csharpInstanceType;
            this.systemObject = systemObject;
            parentCSharpInstance = null;
            childCSharpInstances = new List<ICSharpInstance>();
        }

        public CSharpInstance(object systemObject, ICSharpInstanceType csharpInstanceType, ICSharpInstance parentCSharpInstance)
        {
            csharpInstanceIdentifier = new CSharpInstanceIdentifier((ICSharpInstanceTypeIdentifier)csharpInstanceType.Identifier);
            this.csharpInstanceType = csharpInstanceType;
            this.systemObject = systemObject;
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
