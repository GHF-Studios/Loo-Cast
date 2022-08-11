namespace LooCast.Movement
{
    using LooCast.Target;
    
    public interface ITargetedMovement : IMovement
    {
        Target GetTarget();

        void SetTarget(Target target);
    } 
}
