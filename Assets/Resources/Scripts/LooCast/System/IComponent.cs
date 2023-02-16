namespace LooCast.System
{
    using System.Identification;
    
    public interface IComponent : IInstance
    {
        #region Properties
        IComponentIdentifier ComponentIdentifier { get; }
        #endregion
    }
}