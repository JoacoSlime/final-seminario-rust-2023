# Consigna
El trabajo consiste en desarrollar un sistema en blockchain para el registro de pagos de cuotas mensuales y la gestión de actividades deportivas para los socios del Club SemRust.
El club cuenta con tres categorías de socios: A, B y C. Además, ofrece una variedad de actividades deportivas, como fútbol, gimnasio, básquet, rugby, hockey, natación, tenis y paddle.
Cada categoría de socio tiene asignado un costo mensual diferente para acceder a las actividades deportivas. Los costos de las cuotas mensuales son los siguientes:

- Categoría A: costo mensual de 5000 tokens. Permite asistir al gimnasio y a todos los deportes disponibles en el club.
- Categoría B: costo mensual de 3000 tokens. Permite asistir al gimnasio y a un deporte adicional seleccionado por el socio.
- Categoría C: costo mensual de 2000 tokens. Permite asistir únicamente al gimnasio.

Es importante destacar que los montos pueden actualizarse a medida que pasa el tiempo, por lo que el sistema debe ser capaz de adaptarse a dichos cambios.
Debe proporcionar las siguientes funcionalidades:

1. Registro de un nuevo socio: Al registrar un nuevo socio, se debe seleccionar la categoría correspondiente. Una vez registrado, se crea un pago pendiente con vencimiento en los próximos 10 días.
2. Registro de un pago: Los socios podrán realizar el pago de su cuota mensual indicando su número de identificación (DNI) y el monto pagado. El sistema deberá verificar que el monto pagado corresponda a la categoría del socio y registrar el pago en la blockchain.
3. Consulta de pagos: Se podrá consultar la lista de pagos realizados, mostrando la información del socio, la categoría y el monto pagado.
4. Bonificación por pagos consecutivos: Si un socio ha acumulado un número determinado de pagos consecutivos sin atrasos, el sistema ejecutará una acción de bonificación para ese socio, otorgándole un descuento en la cuota mensual del siguiente mes.

Además, se deben realizar los siguientes reportes a través de otro contrato:

1. Verificación de pagos pendientes: Se deberá mostrar un listado con los socios morosos, es decir, aquellos que tienen pagos pendientes después de la fecha de vencimiento.
2. Generación de informe de recaudación: Se deberá generar un informe de recaudación mensual, mostrando el total recaudado para cada categoría de socio.
3. Generación de un reporte de socios no morosos para una actividad específica: Se deberá generar un reporte de los socios no morosos que tienen permitido asistir a una actividad deportiva específica del club.

En cuanto a los contratos, se debe manejar un listado de direcciones (addresses) autorizadas para realizar operaciones. El dueño del contrato tendrá el poder de autorizar o desautorizar dichas direcciones. Además, se podrá activar o desactivar esta política de autorización por parte del dueño del contrato. Si está desactivada, cualquiera podrá realizar operaciones; si está activada, sólo las direcciones autorizadas podrán hacerlo. El dueño del contrato podrá delegar, lo que permitirá otorgar el poder a otra dirección.
