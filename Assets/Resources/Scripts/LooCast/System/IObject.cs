

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IObject : IInstance, IObjectIdentifiable
    {
        #region Properties
        IInstance ObjectInstance { get; }
        #endregion
    }
}
